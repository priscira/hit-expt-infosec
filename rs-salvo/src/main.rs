mod weibo;
mod dbs;
mod views;
mod exceptions;
mod utils;
mod prefs;

use nyquest::ClientBuilder;
use nyquest::AsyncClient;
use exceptions::NyquestError;
use crate::utils::attain_ajax_hotsearch;

#[tokio::main]
async fn main() {
  nyquest_preset::register();
  let weibo_cli: AsyncClient =
    ClientBuilder::default().base_url("https://weibo.com")
                            .user_agent("Mozilla/5.0 (X11; Linux x86_64) \
                                        AppleWebKit/537.36 (KHTML, like Gecko) \
                                        Chrome/130.0.0.0 Safari/537.36")
                            .dangerously_ignore_certificate_errors()
                            .build_async()
                            .await
                            .expect("Failed to build client");

  let hot_search: Result<String, NyquestError> = weibo::hot::gain_ajax_hotsearch(weibo_cli).await;
  match hot_search {
    | Ok(talk) => {
      if let Err(err) = attain_ajax_hotsearch(&talk) {
        println!("失败: {}", err);
      } else {
        println!("成功");
      }
    },
    | Err(e) => println!("失败: {}", e)
  };
}
