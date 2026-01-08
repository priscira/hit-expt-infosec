from typing import Optional

from piccolo.columns import Integer, Varchar
from piccolo.table import Table


class WeiboHotSearch(Table):
  id = Integer(primary_key=True)
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
    return cls(title=realtime_title, number=realtime_number, special=realtime_special,
               occur_era=occur_era)

  @classmethod
  async def weibo_hot_search_r(
    cls, weibo_title: Optional[str] = None, occur_era: Optional[str] = None
    ) -> list["WeiboHotSearch"]:
    query = cls.select()
    if weibo_title:
      query = query.where(cls.title == weibo_title)
    if occur_era:
      query = query.where(cls.occur_era == occur_era)
    return await query

  @classmethod
  async def weibo_hot_search_u(cls, hot_search_arrs: list["WeiboHotSearch"]) -> None:
    if not hot_search_arrs:
      return

    objects_to_insert = [
      {
        "title": item.title,
        "number": item.number,
        "special": item.special,
        "occur_era": item.occur_era
        }
      for item in hot_search_arrs
      ]

    await cls.insert(*objects_to_insert).on_conflict_do_nothing().run()

  @classmethod
  async def weibo_hot_search_d(
    cls, no_sieve: bool = False, occur_era: Optional[str] = None
    ) -> None:
    if not no_sieve and occur_era is None:
      raise ValueError(
        "delete all the data from the database, but no_sieve guarantee isn't provided.")

    query = cls.delete()
    if occur_era:
      query = query.where(cls.occur_era == occur_era)

    await query.run()
