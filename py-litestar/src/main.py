import hypercorn
from hypercorn.asyncio import serve
from litestar import Litestar
from niquests import AsyncSession


def furnish_niquests_weibo_clt(litestar_app: Litestar):
  weibo_clt: AsyncSession = AsyncSession(
    base_url="https://weibo.com/ajax/")
  weibo_clt.headers = {
    "Referer": "https://weibo.com/newlogin",
    "User-Agent": "Mozilla/5.0 (X11; Linux x86_64) "
                  "AppleWebKit/537.36 (KHTML, like Gecko) "
                  "Chrome/114.0.0.0 Safari/537.36"
    }
  weibo_clt.verify = False
  litestar_app.state.weibo_clt = weibo_clt


async def forsake_niquests_weibo_clt(litestar_app: Litestar):
  if hasattr(litestar_app.state, "weibo_clt"):
    await litestar_app.state.weibo_clt.close()


def furnish_litestar_app():
  return Litestar(
    on_startup=[furnish_niquests_weibo_clt],
    on_shutdown=[forsake_niquests_weibo_clt])


def main():
  import asyncio
  litestar_app = furnish_litestar_app()
  litestar_pref = hypercorn.config.Config()
  litestar_pref.bind = [":::5801"]
  asyncio.run(serve(litestar_app, litestar_pref))
