use nyquest::AsyncClient;
use nyquest::r#async::Request;
use nyquest::r#async::Response;
use crate::exceptions::WeiboError;

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


pub async fn gain_feed_hottimeline(weibo_clt: &AsyncClient) -> Result<String, WeiboError> {
  let gain_info = Request::get("https://weibo.com/ajax/feed/hottimeline?since_id=0&refresh=0&group_id=102803&containerid=102803&extparam=discover%7Cnew_feed&max_id=0&count=10")
    .with_header("x-xsrf-token", "QqDpWsqwA9Prqs7Os_NYw1st")
    .with_header("accept", "application/json, text/plain, */*")
    .with_header(":authority", "weibo.com")
    .with_header(":method", "GET")
    .with_header(":path", "/ajax/feed/hottimeline?since_id=0&refresh=0&group_id=102803&containerid=102803&extparam=discover%7Cnew_feed&max_id=0&count=10")
    .with_header(":scheme", "https")
    .with_header("priority", "u=1, i")
    .with_header("client-version", "v2.47.139")
    .with_header("dnt", "1")
    .with_header("sec-gpc", "1")
    .with_header("server-verion", "v2025.12.08.1")
    .with_header("sec-ch-ua", "\"Not?A_Brand\";v=\"99\", \"Chromium\";v=\"130\"")
    .with_header("sec-ch-ua-mobile", "?0")
    .with_header("sec-ch-ua-platform", "\"Linux\"")
    .with_header("x-requested-with", "XMLHttpRequest")
    .with_header("cookie", "SUB=_2AkMfmjsOf8NxqwFRmvsXyG_mZIt_yQzEieKpxsrVJRMxHRl-yT9kqlA7tRB6NBoV4ZGJe5Iw-S2YDB_0-D8LEMJWYViw; SUBP=0033WrSXqPxfM72-Ws9jqgMF55529P9D9W5Thx-eSSwjIkwC-N8HkqHm; XSRF-TOKEN=QqDpWsqwA9Prqs7Os_NYw1st; _s_tentry=-; Apache=9207789209491.236.1765172221548; SINAGLOBAL=9207789209491.236.1765172221548; ULV=1765172221615:1:1:1:9207789209491.236.1765172221548:; WBPSESS=VFkNPEb725ruVm0iDC4EQHA-M8rztOFecGZtfAyi8nybASutxiBhqHtDGw1xDEREdE4Lw2ModKirA9_38V7YnMWCz96-nz6XXwxwHuhpfPT9CjUzAc1A087bCmiPbsREenNXJDsxjz6OeADOHcCQnN1VtrMl2keHI3oNr8LmIQw=")
    .with_header("referer", "https://weibo.com/newlogin?tabtype=weibo&gid=102803&openLoginLayer=0&url=https%3A%2F%2Fweibo.com%2F");
  let reap: Response = weibo_clt.request(gain_info).await?;
  if !reap.status().is_successful() {
    return Err(WeiboError::NyquestError(format!("/ajax/feed/hottimeline status code is {}",
                                         reap.status().code())));
  }
  let reap_talks = reap.text().await?;
  println!("## reap_talks: {}", reap_talks);
  Ok(reap_talks)
}
