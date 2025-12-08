use nyquest::AsyncClient;
use nyquest::r#async::Request;
use nyquest::r#async::Response;
use crate::exceptions::WeiboError;

pub async fn gain_side_hotsearch(weibo_clt: AsyncClient) -> Result<String, WeiboError> {
  let gain_info = Request::get("/ajax/side/hotSearch");
  // let reap = weibo_clt.request(gain_info).await;
  // if let Err(reap_flaw) = reap {
  //   eprintln!("nyquest error: {:?}", reap_flaw);
  //   return Err(NyquestError::new(format!("nyquest debug flaw info: {}", reap_flaw)));
  // }
  // let reap = reap?;
  let reap: Response = weibo_clt.request(gain_info).await?;
  if !reap.status().is_successful() {
    return Err(WeiboError::NyquestError(format!("/ajax/side/hotSearch status code is {}",
                                         reap.status().code())));
  }
  let reap_talks = reap.text().await?;
  Ok(reap_talks)
}
