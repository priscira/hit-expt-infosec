from typing import Any
from niquests import AsyncResponse, AsyncSession

from .prefs import WEIBO_COK


async def gain_side_hotsearch(weibo_clt: AsyncSession) -> Any:
  """
  访问链接获取微博热搜
  """
  reap: AsyncResponse = await weibo_clt.get("side/hotSearch", stream=True)
  if not reap.ok:
    raise Exception("gain_intensive_order_page, status code is not 200")
  return await reap.json()


async def gain_feed_hottimeline(weibo_clt: AsyncSession) -> str:
  """
  访问链接获取微博热门推荐
  """
  reap: AsyncResponse = await weibo_clt.get(
    "feed/hottimeline",
    params={
      "since_id": 0, "refresh": 0, "group_id": "102803", "containerid": "102803",
      "extparam": "discover|new_feed", "max_id": 0, "count": 10},
    cookies=WEIBO_COK,
    stream=True)
  if not reap.ok:
    raise Exception(f"/ajax/feed/hottimeline status code is {reap.status_code}")
  return await reap.json()


async def gain_sinaimg(weibo_clt: AsyncSession, pic_url: str) -> bytes:
  """
  访问链接获取微博图片
  """
  reap: AsyncResponse = await weibo_clt.get(pic_url, stream=True)
  if not reap.ok:
    raise Exception(f"{pic_url} status code is {reap.status_code}")
  return await reap.content


async def gain_status_build_comments(weibo_clt: AsyncSession, mid: str, uid: str) -> str:
  """
  访问链接获取微博热门推荐
  """
  reap: AsyncResponse = await weibo_clt.get(
    "statuses/buildComments",
    params={
      "is_reload": 1, "id": mid, "is_show_bulletin": 2, "is_mix": 0,
      "count": 20, "type": "feed", "uid": uid, "fetch_level": 0, "locale": "zh-CN"},
    cookies=WEIBO_COK,
    stream=True)
  if not reap.ok:
    raise Exception(f"/ajax/statuses/buildComments status code is {reap.status_code}")
  return await reap.json()
