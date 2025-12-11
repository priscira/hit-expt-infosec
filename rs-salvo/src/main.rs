mod dbs;
mod exceptions;
mod prefs;
mod utils;
mod views;
mod weibo;
mod wm;

use nyquest::ClientBuilder;
use nyquest::AsyncClient;
use crate::utils::{attain_ajax_hotsearch, attain_ajax_hottimeline};
use crate::weibo::gain_sinaimg;
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
      .build_async()
      .await
      .expect("Failed to build client");
  // gain_sinaimg(&weibo_clt, "https://wx2.sinaimg.cn/orj360/79510c8bgy1i85kbqe8v3j222e35ekgp.jpg").await.unwrap();
  match attain_ajax_hottimeline(&weibo_clt, true).await {
    | Ok(_) => {
      println!("成功");
    }
    | Err(flaw) => println!("失败: {}", flaw)
  };
}
