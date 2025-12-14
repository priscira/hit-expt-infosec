mod dbs;
mod exceptions;
mod prefs;
mod utils;
mod weibo;
mod wm;
mod views;

use nyquest::AsyncClient;
use nyquest::ClientBuilder;
use rbatis::RBatis;
use rbdc_sqlite::SqliteDriver;
use salvo::prelude::*;
use salvo_mdw::LogLogger;
use crate::prefs::WEIBO_DB_PTH;
use crate::views::*;

// TODO: 修改weibo的cookie

#[tokio::main]
async fn main() {
  // log4rs日志初始化
  log4rs::init_file("weibo-log4rs.yml", Default::default()).expect("log4rs: failed to init logger");

  // nyquest爬虫客户端
  nyquest_preset::register();
  let weibo_clt: AsyncClient =
    ClientBuilder::default().base_url("https://weibo.com/ajax/").
      with_header("Referer", "https://weibo.com/newlogin").
      user_agent("Mozilla/5.0 (X11; Linux x86_64) \
                  AppleWebKit/537.36 (KHTML, like Gecko) \
                  Chrome/130.0.0.0 Safari/537.36").
      dangerously_ignore_certificate_errors().no_redirects().
      build_async().await.
      expect("nyquest: failed to build async-client");

  // rbatis数据库连接
  let weibo_db_rb_conn: RBatis = RBatis::new();
  weibo_db_rb_conn.link(SqliteDriver {}, WEIBO_DB_PTH).await.
    expect("rbatis: failed to link sqlite");

  let salvo_accept = TcpListener::new("0.0.0.0:5800").bind().await;
  let salvo_rt = Router::new().
    hoop(affix_state::insert("weibo_clt", weibo_clt).
      insert("weibo_db_rb_conn", weibo_db_rb_conn)).
    hoop(CatchPanic::new()).
    get(hello);
  let salvo_svc = Service::new(salvo_rt).hoop(LogLogger::new());
  Server::new(salvo_accept).serve(salvo_svc).await;
}
