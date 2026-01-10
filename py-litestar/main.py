import hypercorn
from hypercorn.asyncio import serve
from litestar import Litestar, MediaType, Request, Response
from litestar.exceptions import LitestarException
from litestar.status_codes import HTTP_200_OK, HTTP_500_INTERNAL_SERVER_ERROR
from niquests import AsyncSession
from src import exceptions, views


def tackle_litestar_expt(_: Request, expt: Exception) -> Response:
  expt_ctn = getattr(expt, "detail", "")

  return Response(
    media_type=MediaType.JSON,
    content=views.RespBdM.err_resp(expt_ctn),
    status_code=HTTP_200_OK,
    )


def tackle_weibo_expt(_: Request, expt: exceptions.WeiboException) -> Response:
  expt_ctn = ""
  match type(expt):
    case exceptions.WeiboLitestarException:
      expt_ctn = "service error"
    case exceptions.WeiboPiccoloException:
      expt_ctn = "database error"
    case exceptions.WeiboNiquestsException:
      expt_ctn = "cannot access weibo"
    case exceptions.WeiboMarshmallowException:
      expt_ctn = "analyse error"
    case _:
      expt_ctn = "internal error"

  return Response(
    media_type=MediaType.JSON,
    content=views.RespBdM.err_resp(expt_ctn),
    status_code=HTTP_200_OK,
    )


def furnish_niquests_weibo_clt(litestar_app: Litestar):
  weibo_clt: AsyncSession = AsyncSession(base_url="https://weibo.com/ajax/", headers={
    "Referer": "https://weibo.com/newlogin",
    "User-Agent": "Mozilla/5.0 (X11; Linux x86_64) "
                  "AppleWebKit/537.36 (KHTML, like Gecko) "
                  "Chrome/114.0.0.0 Safari/537.36"
    })
  weibo_clt.stream = True
  weibo_clt.verify = False
  litestar_app.state.weibo_clt = weibo_clt


async def forsake_niquests_weibo_clt(litestar_app: Litestar):
  if hasattr(litestar_app.state, "weibo_clt"):
    await litestar_app.state.weibo_clt.close()


# sqlite不支持connection_pool
# async def furnish_piccolo_conn():
#   db_eng = engine_finder()
#   await db_eng.start_connection_pool()
#
#
# async def forsake_piccolo_conn():
#   db_eng = engine_finder()
#   await db_eng.close_connection_pool()


def furnish_litestar_app() -> Litestar:
  return Litestar(
    exception_handlers={
      LitestarException: tackle_litestar_expt,
      HTTP_500_INTERNAL_SERVER_ERROR: tackle_weibo_expt},
    on_startup=[furnish_niquests_weibo_clt],
    on_shutdown=[forsake_niquests_weibo_clt],
    route_handlers=[
      views.weibo_controller_r_ctrl,
      views.weibo_controller_u_ctrl,
      views.weibo_controller_d_ctrl,
      ])


def main():
  import asyncio
  litestar_app = furnish_litestar_app()
  litestar_pref = hypercorn.config.Config()
  litestar_pref.bind = [":::5800"]
  asyncio.run(serve(litestar_app, litestar_pref))


if __name__ == "__main__":
  main()
