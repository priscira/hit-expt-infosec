from datetime import datetime
from typing import Any
from marshmallow import fields, Schema, post_load
from marshmallow.experimental.context import Context
from niquests import AsyncSession
from . import weibo
from .exceptions import WeiboMarshmallowException, WeiboNiquestsException
from .dbs import WeiboHotSearch, WeiboHotTimeline


class WeiboHotSearchM(Schema):
  # 热搜标题
  word = fields.Str(required=True)
  # 热搜热度
  num = fields.Int(load_default=0)
  # 热搜是否为特殊热搜，比如“热”、“新”
  label_name = fields.Str(load_default="")

  @post_load
  def furnish_piccolo_info(self, info: dict, **_kwargs) -> WeiboHotSearch:
    occur_era = Context.get()["occur_era"]
    return WeiboHotSearch.weibo_hot_search_c(
      realtime_title=info["word"],
      realtime_number=info["num"],
      realtime_special=info["label_name"],
      occur_era=occur_era
      )


class WeiboHotTimelineM(Schema):
  class WeiboHotTimelineUserM(Schema):
    """热门推荐的用户信息"""
    idstr = fields.Str(required=True)
    screen_name = fields.Str(required=True)

  # 热门推荐的mid
  mid = fields.Str(required=True)
  # 热门推荐的mblogid
  mblogid = fields.Str(required=True)
  # 热门推荐的内容
  text_raw = fields.Str(required=True)
  # 热门推荐的发布者
  user = fields.Nested(WeiboHotTimelineUserM(unknown="exclude"), required=True)
  # 热门推荐出现的时间
  created_at = fields.Str(required=False)

  @post_load
  def furnish_piccolo_info(self, info: dict, **_kwargs) -> WeiboHotTimeline:
    occur_era = info["created_at"]
    if occur_era:
      occur_era = datetime.strptime(occur_era, "%a %b %d %H:%M:%S %z %Y").strftime("%Y-%m-%d")
    else:
      occur_era = datetime.now().strftime("%Y-%m-%d")

    return WeiboHotTimeline.weibo_hot_timeline_c(
      timeline_mid=info["mid"],
      timeline_mblogid=info["mblogid"],
      timeline_text=info["text_raw"],
      timeline_mem_id=info["user"]["idstr"],
      timeline_mem_name=info["user"]["screen_name"],
      occur_era=occur_era
      )


weibo_hot_search_m = WeiboHotSearchM(unknown="exclude")
weibo_hot_timeline_m = WeiboHotTimelineM(unknown="exclude")


async def attain_ajax_hotsearch(weibo_clt: AsyncSession) -> None:
  hot_search_jquin: Any = await weibo.gain_side_hotsearch(weibo_clt)
  hot_search_realtime_arrs: list[dict[str, Any]] = []
  match hot_search_jquin:
    case {"data": {"realtime": list(hot_search_realtime_arrs)}}:
      hot_search_realtime_arrs = hot_search_realtime_arrs
    case _:
      raise WeiboNiquestsException("/ajax/side/hotSearch, response format error")
  hot_search_arrs: list[WeiboHotSearch] = []
  occur_era = datetime.now().strftime("%Y-%m-%d")
  with Context({"occur_era": occur_era}):
    for hot_search_realtime_arri in hot_search_realtime_arrs:
      try:
        if hot_search_realtime_arri.get("is_ad") == 1:
          continue
        hot_search_realtime_info = weibo_hot_search_m.load(hot_search_realtime_arri)
        hot_search_arrs.append(hot_search_realtime_info)
      except Exception as e:
        raise WeiboMarshmallowException(str(e)) from e

  await WeiboHotSearch.weibo_hot_search_u(hot_search_arrs)


async def attain_ajax_hottimeline(weibo_clt: AsyncSession, pic: bool, comm: bool) -> None:
  hot_timeline_jquin: Any = await weibo.gain_feed_hottimeline(weibo_clt)
  hot_timeline_status_arrs: list[dict[str, Any]] = []
  match hot_timeline_jquin:
    case {"statuses": list(hot_timeline_status_arrs)}:
      hot_timeline_status_arrs = hot_timeline_status_arrs
    case _:
      raise WeiboNiquestsException("/ajax/feed/hottimeline, response format error")
  hot_timelime_arrs: list[WeiboHotTimeline] = []
  for hot_timeline_status_arri in hot_timeline_status_arrs:
    try:
      hot_timeline_info = weibo_hot_timeline_m.load(hot_timeline_status_arri)
      hot_timelime_arrs.append(hot_timeline_info)
    except Exception as e:
      print(e)
      raise WeiboMarshmallowException(str(e)) from e

  await WeiboHotTimeline.weibo_hot_timeline_u(hot_timelime_arrs)
