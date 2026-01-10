from typing import Any
from litestar import Controller, post, Request, Router
from litestar.datastructures.state import ImmutableState
from marshmallow import fields, Schema, validates_schema
from niquests import AsyncSession
from .dbs import WeiboHotSearch, WeiboHotTimeline
from .exceptions import WeiboLitestarException, WeiboMarshmallowException
from .utils import attain_ajax_hotsearch, attain_ajax_hottimeline, weibo_hot_timeline_m


class RespBdM(Schema):
  info = fields.Raw(allow_none=True, required=True)
  suc = fields.Bool(required=True)

  @classmethod
  def suc_resp(cls, resp_info: Any):
    """成功响应"""
    return cls().dump({"info": resp_info, "suc": True})

  @classmethod
  def err_resp(cls, err_info: str):
    """失败响应"""
    return cls().dump({"info": err_info, "suc": False})


class WeiboControllerR(Controller):
  class ReqBdHotSearchR(Schema):
    weibo_title = fields.Str(allow_none=True, required=True)
    occur_era = fields.Str(allow_none=True, required=True)

  class ReqBdHotTimelineR(Schema):
    timeline_mid = fields.Str(allow_none=True, required=True)
    timeline_mem_id = fields.Str(allow_none=True, required=True)
    timeline_mem_name = fields.Str(allow_none=True, required=True)
    timeline_occur_era = fields.Str(allow_none=True, required=True)
    pic = fields.Bool(required=True)
    comm = fields.Bool(required=True)

  reqbd_hot_search_r_m = ReqBdHotSearchR(unknown="exclude")
  reqbd_hot_timeline_r_m = ReqBdHotTimelineR(unknown="exclude")

  @post("/hot_search")
  async def hot_search_r(self, request: Request) -> dict:
    try:
      req_info = self.reqbd_hot_search_r_m.load(await request.json())
    except Exception as e:
      raise WeiboMarshmallowException(str(e)) from e
    weibo_hot_search_arrs = await WeiboHotSearch.weibo_hot_search_r(**req_info)
    hot_search_arrs = [
      weibo_hot_search_arri.to_dict() for weibo_hot_search_arri in weibo_hot_search_arrs]
    return RespBdM.suc_resp(hot_search_arrs)

  @post("/hot_timeline")
  async def hot_timeline_r(self, request: Request) -> dict:
    try:
      req_info = self.reqbd_hot_timeline_r_m.load(await request.json())
    except Exception as e:
      raise WeiboMarshmallowException(str(e)) from e
    weibo_hot_timeline_arrs = await WeiboHotTimeline.weibo_hot_timeline_r(**req_info)
    hot_timeline_arrs = [
      weibo_hot_timeline_arri.to_dict() for weibo_hot_timeline_arri in weibo_hot_timeline_arrs]
    return RespBdM.suc_resp(hot_timeline_arrs)


class WeiboControllerU(Controller):
  class ReqBdHotTimelineU(Schema):
    timeline_pic = fields.Bool(required=True)
    timeline_comm = fields.Bool(required=True)

  reqbd_hot_timeline_u_m = ReqBdHotTimelineU(unknown="exclude")

  @post("/hot_search")
  async def hot_search_u(self, state: ImmutableState) -> dict:
    weibo_clt: AsyncSession = state.get("weibo_clt")
    if not weibo_clt:
      raise WeiboLitestarException("cannot establish weibo client")
    await attain_ajax_hotsearch(weibo_clt)
    return RespBdM.suc_resp("ok")

  @post("/hot_timeline")
  async def hot_timeline_u(self, request: Request, state: ImmutableState) -> dict:
    try:
      req_info = self.reqbd_hot_timeline_u_m.load(await request.json())
    except Exception as e:
      raise WeiboMarshmallowException(str(e)) from e
    weibo_clt: AsyncSession = state.get("weibo_clt")
    if not weibo_clt:
      raise WeiboLitestarException("cannot establish weibo client")
    await attain_ajax_hottimeline(weibo_clt,
                                  pic=req_info["timeline_pic"], comm=req_info["timeline_comm"])
    return RespBdM.suc_resp("ok")


class WeiboControllerD(Controller):
  class ReqBdHotSearchD(Schema):
    no_sieve = fields.Bool(allow_none=False, required=True)
    occur_era = fields.Str(allow_none=True, required=True)

    @validates_schema
    def judge_sieve(self, data, **_kwargs):
      if not data["occur_era"] and data["no_sieve"] is not True:
        raise WeiboMarshmallowException(
          "delete all the data from the database, but no_sieve guarantee isn't provided.")

  class ReqBdHotTimelineD(Schema):
    no_sieve = fields.Bool(allow_none=False, required=True)
    timeline_mid = fields.Str(allow_none=True, required=True)
    timeline_mem_id = fields.Str(allow_none=True, required=True)
    timeline_mem_name = fields.Str(allow_none=True, required=True)
    occur_era = fields.Str(allow_none=True, required=True)

    @validates_schema
    def judge_sieve(self, data, **_kwargs):
      yes_sieves = ["timeline_mid", "timeline_mem_id", "timeline_mem_name"]
      if data["no_sieve"] is False and all(not data[yes_sievei] for yes_sievei in yes_sieves):
        raise WeiboMarshmallowException(
          "delete all the data from the database, but no_sieve guarantee isn't provided.")

  reqbd_hot_search_d_m = ReqBdHotSearchD(unknown="exclude")
  reqbd_hot_timeline_d_m = ReqBdHotTimelineD(unknown="exclude")

  @post("/hot_search")
  async def hot_search_d(self, request: Request) -> dict:
    req_info = self.reqbd_hot_search_d_m.load(await request.json())
    await WeiboHotSearch.weibo_hot_search_d(**req_info)
    return RespBdM.suc_resp("ok")

  @post("/hot_timeline")
  async def hot_timeline_d(self, request: Request) -> dict:
    req_info = self.reqbd_hot_search_d_m.load(await request.json())
    await WeiboHotSearch.weibo_hot_search_d(**req_info)
    return RespBdM.suc_resp("ok")


weibo_controller_r_ctrl = Router(path="/r", route_handlers=[WeiboControllerR])
weibo_controller_u_ctrl = Router(path="/u", route_handlers=[WeiboControllerU])
weibo_controller_d_ctrl = Router(path="/d", route_handlers=[WeiboControllerD])
