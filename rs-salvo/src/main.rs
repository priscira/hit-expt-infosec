mod dbs;
mod exceptions;
mod prefs;
mod utils;
mod weibo;
mod wm;

use nyquest::AsyncClient;
use nyquest::ClientBuilder;
use crate::utils::*;

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
  // mid: 5242751438294893
  // uid: 7277232894
  match furnish_ajax_comments_hot_timeline(&weibo_clt, "5242751438294893", "7277232894").await {
    | Ok(_) => {
      println!("成功");
    }
    | Err(flaw) => println!("失败: {}", flaw)
  };
}
