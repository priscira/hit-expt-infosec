use nyquest::AsyncClient;
use nyquest::r#async::Request;
use nyquest::r#async::Response;
use crate::exceptions::WeiboError;

/// 访问链接获取微博热搜
pub async fn gain_side_hotsearch(weibo_clt: &AsyncClient) -> Result<String, WeiboError> {
  let gain_info = Request::get("side/hotSearch");
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

/// 访问链接获取微博热门推荐
pub async fn gain_feed_hottimeline(weibo_clt: &AsyncClient) -> Result<String, WeiboError> {
  let gain_info = Request::get("https://weibo.com/ajax/feed/hottimeline?\
                                since_id=0&refresh=0&group_id=102803&containerid=102803&\
                                extparam=discover|new_feed&max_id=0&count=10")
    .with_header("cookie", "SUB=_2AkMfmjsOf8NxqwFRmvsXyG_mZIt_yQzEieKpxsrVJRMxH\
                                Rl-yT9kqlA7tRB6NBoV4ZGJe5Iw-S2YDB_0-D8LEMJWYViw");
  let reap: Response = weibo_clt.request(gain_info).await?;
  if !reap.status().is_successful() {
    return Err(WeiboError::NyquestError(format!("/ajax/feed/hottimeline status code is {}",
                                                reap.status().code())));
  }
  let reap_talks = reap.text().await?;
  Ok(reap_talks)
}

/// 访问链接获取微博图片
pub async fn gain_sinaimg(weibo_clt: &AsyncClient, pic_url: &str) -> Result<Vec<u8>, WeiboError> {
  let gain_info = Request::get(pic_url.to_string());
  let reap: Response = weibo_clt.request(gain_info).await?;
  if !reap.status().is_successful() {
    return Err(WeiboError::NyquestError(format!("{} status code is {}",
                                                pic_url, reap.status().code())));
  }
  let reap_byt: Vec<u8> = reap.bytes().await?;
  Ok(reap_byt)
}
