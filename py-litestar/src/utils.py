from datetime import datetime
from typing import Any
from marshmallow import fields, Schema, post_load
from marshmallow.experimental.context import Context
from niquests import AsyncSession
from . import weibo
from .dbs import WeiboHotSearch


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


async def attain_ajax_hotsearch(weibo_clt: AsyncSession):
  hot_search_jquin: Any = await weibo.gain_side_hotsearch(weibo_clt)
  hot_search_realtime_arrs: list[dict[str, Any]] = []
  match hot_search_jquin:
    case {"data": {"realtime": list(hot_search_realtime_arrs)}}:
      hot_search_realtime_arrs = hot_search_realtime_arrs
    case _:
      raise Exception("gain_side_hotsearch, response is not dict")
  hot_search_arrs: list[WeiboHotSearch] = []
  weibo_hot_search_m = WeiboHotSearchM(unknown="exclude")
  occur_era = datetime.now().strftime("%Y-%m-%d")
  with Context({"occur_era": occur_era}):
    for hot_search_realtime_arri in hot_search_realtime_arrs:
      if hot_search_realtime_arri.get("is_ad") == 1:
        continue
      hot_search_realtime_info = weibo_hot_search_m.load(hot_search_realtime_arri)
      hot_search_arrs.append(hot_search_realtime_info)

  # await WeiboHotSearch.weibo_hot_search_u(hot_search_arrs)
