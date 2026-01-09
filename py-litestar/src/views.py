from litestar import Controller, post, Request, Router
from litestar.datastructures.state import ImmutableState
from marshmallow import fields, Schema
from niquests import AsyncSession
from .dbs import WeiboHotSearch
from .exceptions import LitestarException
from .utils import attain_ajax_hotsearch


class WeiboControllerR(Controller):
  class ReqBdHotSearchR(Schema):
    weibo_title = fields.Str(allow_none=True, required=True)
    occur_era = fields.Str(allow_none=True, required=True)

  reqbd_hot_search_r_m = ReqBdHotSearchR(unknown="exclude")

  @post("/hot_search")
  async def hot_search_r(self, request: Request, state: ImmutableState) -> list[dict[str, str]]:
    req_info = self.reqbd_hot_search_r_m.load(await request.json())
    weibo_clt: AsyncSession = state.get("weibo_clt")
    if not weibo_clt:
      raise LitestarException("cannot establish weibo client")
    weibo_hot_search_arrs = await WeiboHotSearch.weibo_hot_search_r(**req_info)
    return [weibo_hot_search_arri.to_dict() for weibo_hot_search_arri in weibo_hot_search_arrs]


class WeiboControllerU(Controller):
  @post("/hot_search")
  async def hot_search_u(self):
    pass


class WeiboControllerD(Controller):
  @post("/hot_search")
  async def hot_search_d(self):
    pass


weibo_controller_r_ctrl = Router(path="/r", route_handlers=[WeiboControllerR])
