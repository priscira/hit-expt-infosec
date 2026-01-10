from typing import Optional
from piccolo.columns import Integer, Varchar
from piccolo.table import Table
from .exceptions import WeiboPiccoloException


class WeiboHotSearch(Table, tablename="weibo_hot_search"):
  id = Integer(null=True, primary_key=True)
  # 热搜标题
  title = Varchar()
  # 热搜热度
  number = Integer()
  # 热搜是否为特殊热搜，比如“热”、“新”
  special = Varchar()
  # 热搜出现的时间，格式为YYYY-MM-DD
  occur_era = Varchar()

  @classmethod
  def weibo_hot_search_c(cls, realtime_title: str, realtime_number: int, realtime_special: str,
                         occur_era: str) -> "WeiboHotSearch":
    """
    创建一个微博热搜WeiboHotSearch对象。

    Parameters
    ----------
    realtime_title: str
      热搜标题
    realtime_number: int
      热搜热度
    realtime_special: str
      热搜是否为特殊热搜，比如“热”、“新”
    occur_era: str
      热搜出现的时间，格式为YYYY-MM-DD

    Returns
    -------
    WeiboHotSearch
      微博热搜对象
    """
    # 需要显式指定id=None，否则默认0
    return cls(id=None, title=realtime_title, number=realtime_number, special=realtime_special,
               occur_era=occur_era)

  @classmethod
  async def weibo_hot_search_r(cls,
                               weibo_title: Optional[str] = None, occur_era: Optional[str] = None
                               ) -> list["WeiboHotSearch"]:
    """
    获取微博热搜WeiboHotSearch对象。

    Parameters
    ----------
    weibo_title: str | None
      热搜标题，可选
    occur_era: str | None
      热搜出现的年月日，格式YYYY-MM-DD，可选

    Returns
    -------
    list[WeiboHotSearch]
      微博热搜对象列表
    """
    # objects()的表现在意料之外
    weibo_hot_search_r_qry = cls.objects()
    if weibo_title:
      weibo_hot_search_r_qry = weibo_hot_search_r_qry.where(cls.title == weibo_title)
    if occur_era:
      weibo_hot_search_r_qry = weibo_hot_search_r_qry.where(cls.occur_era == occur_era)
    return await weibo_hot_search_r_qry.run()

  @classmethod
  async def weibo_hot_search_u(cls, hot_search_arrs: list["WeiboHotSearch"]) -> None:
    """
    更新微博热搜WeiboHotSearch数据，如果有当天的同名的热搜，那么不做处理；否则直接插入。

    Parameters
    ----------
    hot_search_arrs: list[WeiboHotSearch]
      新的微博热搜数据

    Returns
    -------
    None
    """
    if not hot_search_arrs:
      return
    await cls.insert(*hot_search_arrs).on_conflict(action="DO NOTHING").run()

  @classmethod
  async def weibo_hot_search_d(cls, no_sieve: bool = False,
                               occur_era: Optional[str] = None) -> None:
    """
    删除微博热搜WeiboHotSearch数据。

    Parameters
    ----------
    no_sieve: bool
      无筛选条件，删除全部数据时的保证参数
    occur_era: str | None
      热搜出现的年月日，格式YYYY-MM-DD，可选

    Returns
    -------
    None

    Raises
    ------
    PiccoloException
      无条件清空数据库却未保证no_sieve参数
    """
    if not no_sieve and occur_era is None:
      raise WeiboPiccoloException(
        "delete all the data from the database, but no_sieve guarantee isn't provided.")

    if not no_sieve:
      weibo_hot_search_d_qry = cls.delete()
      if occur_era:
        weibo_hot_search_d_qry = weibo_hot_search_d_qry.where(cls.occur_era == occur_era)
    else:
      weibo_hot_search_d_qry = cls.delete(force=True)

    await weibo_hot_search_d_qry.run()


class WeiboHotTimeline(Table, tablename="weibo_hot_timeline"):
  id = Integer(null=True, primary_key=True)
  # 热门推荐的mid
  mid = Varchar()
  # 热门推荐的mblogid
  mblogid = Varchar()
  # 热门推荐的内容
  text = Varchar()
  # 热门推荐的发布者的编号
  mem_id = Varchar()
  # 热门推荐的发布者的名称
  mem_name = Varchar()
  # 热门推荐出现的时间，格式为YYYY-MM-DD
  occur_era = Varchar()

  @classmethod
  def weibo_hot_timeline_c(cls, timeline_mid: str, timeline_mblogid: int, timeline_text: str,
                           timeline_mem_id: str, timeline_mem_name: int,
                           occur_era: str) -> "WeiboHotTimeline":
    """
    创建一个微博热门推荐WeiboHotTimeline对象。

    Parameters
    ----------
    timeline_mid: str
      热门推荐的mid
    timeline_mblogid: int
      热门推荐的mblogid
    timeline_text: str
      热门推荐的内容
    timeline_mem_id: str
      热门推荐的发布者的编号
    timeline_mem_name: str
      热门推荐的发布者的名称
    occur_era: str
      热门推荐出现的时间，格式为YYYY-MM-DD

    Returns
    -------
    WeiboHotTimeline
      微博热搜对象
    """
    # 需要显式指定id=None，否则默认0
    return cls(id=None, mid=timeline_mid, mblogid=timeline_mblogid, text=timeline_text,
               mem_id=timeline_mem_id, mem_name=timeline_mem_name, occur_era=occur_era)

  @classmethod
  async def weibo_hot_timeline_r(cls, timeline_mid: Optional[str] = None,
                                 timeline_mem_id: Optional[str] = None,
                                 timeline_mem_name: Optional[str] = None,
                                 timeline_occur_era: Optional[str] = None,
                                 pic: bool = False, comm: bool = False) -> list["WeiboHotTimeline"]:
    """
    获取微博热门推荐WeiboHotTimeline对象。

    Parameters
    ----------
    timeline_mid: str | None
      热门推荐的mid，可选
    timeline_mem_id: str | None
      热门推荐的发布者的编号，可选
    timeline_mem_name: str | None
      热门推荐的发布者的名称，可选
    timeline_occur_era: str | None
      热门推荐出现的时间，格式YYYY-MM-DD，可选
    pic: bool
      是否查询图片
    comm: bool
      是否查询评论

    Returns
    -------
    list[WeiboHotTimeline]
      微博热门推荐WeiboHotTimeline对象
    """
    # objects()的表现在意料之外
    weibo_hot_timeline_r_qry = cls.objects()
    if timeline_mid:
      weibo_hot_timeline_r_qry = weibo_hot_timeline_r_qry.where(cls.mid == timeline_mid)
    if timeline_mem_id:
      weibo_hot_timeline_r_qry = weibo_hot_timeline_r_qry.where(cls.mem_id == timeline_mem_id)
    if timeline_mem_name:
      weibo_hot_timeline_r_qry = weibo_hot_timeline_r_qry.where(cls.mem_name == timeline_mem_name)
    if timeline_occur_era:
      weibo_hot_timeline_r_qry = weibo_hot_timeline_r_qry.where(cls.occur_era == timeline_occur_era)
    return await weibo_hot_timeline_r_qry.run()

  @classmethod
  async def weibo_hot_timeline_u(cls, hot_timeline_arrs: list["WeiboHotTimeline"]) -> None:
    """
    更新微博热门推荐WeiboHotTimeline数据，如果有相同的mid则更新；否则直接插入。

    Parameters
    ----------
    hot_timeline_arrs: list[WeiboHotTimeline]
      新的微博热门推荐数据

    Returns
    -------
    None
    """
    if not hot_timeline_arrs:
      return
    await cls.insert(*hot_timeline_arrs).on_conflict(action="DO NOTHING").run()

  @classmethod
  async def weibo_hot_timeline_d(cls, no_sieve: bool = False,
                                 timeline_mid: Optional[str] = None,
                                 timeline_mem_id: Optional[str] = None,
                                 timeline_mem_name: Optional[str] = None,
                                 timeline_occur_era: Optional[str] = None, ) -> None:
    """
    删除微博热门推荐WeiboHotTimeline数据。

    Parameters
    ----------
    no_sieve: bool
      无筛选条件，删除全部数据时的保证参数
    timeline_mid: str | None
      热门推荐的mid，可选
    timeline_mem_id: str | None
      热门推荐的发布者的编号，可选
    timeline_mem_name: str | None
      热门推荐的发布者的名称，可选
    timeline_occur_era: str | None
      热门推荐出现的时间，格式YYYY-MM-DD，可选

    Returns
    -------
    None

    Raises
    ------
    PiccoloException
      无条件清空数据库却未保证no_sieve参数
    """
    yes_sieve = False
    weibo_hot_search_d_qry = cls.delete()
    if timeline_mid:
      weibo_hot_search_d_qry = weibo_hot_search_d_qry.where(cls.mid == timeline_mid)
      yes_sieve = True
    if timeline_mem_id:
      weibo_hot_search_d_qry = weibo_hot_search_d_qry.where(cls.mem_id == timeline_mem_id)
      yes_sieve = True
    if timeline_mem_name:
      weibo_hot_search_d_qry = weibo_hot_search_d_qry.where(cls.mem_name == timeline_mem_name)
      yes_sieve = True
    if timeline_occur_era:
      weibo_hot_search_d_qry = weibo_hot_search_d_qry.where(cls.occur_era == timeline_occur_era)
      yes_sieve = True

    if not no_sieve and not yes_sieve:
      raise WeiboPiccoloException(
        "delete all the data from the database, but no_sieve guarantee isn't provided.")

    if no_sieve and not yes_sieve:
      weibo_hot_search_d_qry = cls.delete(force=True)

    await weibo_hot_search_d_qry.run()
