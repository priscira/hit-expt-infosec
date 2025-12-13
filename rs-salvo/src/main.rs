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
use crate::prefs::WEIBO_DB_PTH;
use crate::utils::*;
use crate::views::*;

// TODO: 修改weibo的cookie

#[tokio::main]
async fn main() {
  nyquest_preset::register();
  let weibo_clt: AsyncClient =
    ClientBuilder::default().base_url("https://weibo.com/ajax/")
      .with_header("Referer", "https://weibo.com/newlogin")
      .user_agent("Mozilla/5.0 (X11; Linux x86_64) \
                   AppleWebKit/537.36 (KHTML, like Gecko) \
                   Chrome/130.0.0.0 Safari/537.36")
      .dangerously_ignore_certificate_errors()
      .no_redirects()
      .build_async()
      .await
      .expect("Failed to build client");
  let weibo_db_rb_conn = RBatis::new();
  weibo_db_rb_conn.link(SqliteDriver {}, WEIBO_DB_PTH).await.unwrap();

  let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;
  let router = Router::new().hoop(affix_state::insert("weibo_clt", weibo_clt).
    insert("weibo_db_rb_conn", weibo_db_rb_conn)).
    get(hello);
  Server::new(acceptor).serve(router).await;
  // match attain_ajax_hottimeline(&weibo_clt, true, true).await {
  //   Ok(_) => {
  //     println!("成功");
  //   }
  //   Err(flaw) => println!("失败: {}", flaw)
  // };
}
