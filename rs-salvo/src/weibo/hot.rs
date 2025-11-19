use nyquest::AsyncClient;
use nyquest::r#async::Request;
use nyquest::r#async::Response;
use crate::exceptions::NyquestError;

pub async fn gain_ajax_hotsearch(weibo_cli: AsyncClient) -> Result<String, NyquestError> {
  let gain_info =
    Request::get("/ajax/side/hotSearch").with_header("Referer", "https://weibo.com/newlogin");
  // let reap = weibo_cli.request(gain_info).await;
  // if let Err(reap_flaw) = reap {
  //   eprintln!("nyquest error: {:?}", reap_flaw);
  //   return Err(NyquestError::new(format!("nyquest debug flaw info: {}", reap_flaw)));
  // }
  // let reap = reap?;
  let reap: Response = weibo_cli.request(gain_info).await?;
  if !reap.status().is_successful() {
    return Err(NyquestError::new(format!("/ajax/side/hotSearch status code is {}",
                                         reap.status().code())));
  }
  let reap_talks = reap.text().await?;
  Ok(reap_talks)
}
