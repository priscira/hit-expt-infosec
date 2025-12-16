use jzon::object;
use jzon::JsonValue;
use nyquest::AsyncClient;
use rbatis::RBatis;
use salvo::http::header::CONTENT_TYPE;
use salvo::http::HeaderValue;
use salvo::prelude::*;
use crate::dbs::*;
use crate::exceptions::WeiboError;
use crate::utils;

#[handler]
pub async fn hello() -> String {
  "Hello World".to_string()
}

/// 统一的响应结构
///
/// > 想搞点好玩的，封装一个较为底层的回复结构。由于不使用serde-json，需要封装不少。
#[derive(Debug)]
struct RespBd {
  // 响应内容
  info: Option<JsonValue>,
  // 程序是否成功
  suc: bool,
}

impl RespBd {
  /// 成功响应
  ///
  /// ## 参数
  /// - `resp_info`: 响应内容
  pub fn suc_resp(resp_info: impl Into<JsonValue>) -> Self {
    Self {
      info: Some(resp_info.into()),
      suc: true,
    }
  }

  /// 失败响应
  ///
  /// ## 参数
  /// - `err`: 错误信息
  pub fn err_resp(err: WeiboError) -> Self {
    Self {
      info: Some(err.into()),
      suc: false,
    }
  }
}

#[async_trait]
impl Writer for RespBd {
  async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.status_code(StatusCode::OK);
    res.headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json; charset=utf-8"));
    res.render(jzon::stringify(self));
  }
}

#[async_trait]
impl Writer for WeiboError {
  async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.status_code(StatusCode::OK);
    res.headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json; charset=utf-8"));
    res.render(jzon::stringify(RespBd::err_resp(self)));
  }
}

impl Into<JsonValue> for RespBd {
  fn into(self) -> JsonValue {
    object! {
      info: self.info,
      suc: self.suc
    }
  }
}

impl Into<JsonValue> for WeiboError {
  fn into(self) -> JsonValue {
    self.to_string().into()
  }
}

#[handler]
pub async fn hot_search_r(req: &mut Request, depot: &mut Depot) -> Result<RespBd, WeiboError> {
  #[derive(Debug, serde::Deserialize)]
  struct ReqBdHotSearchR {
    weibo_title: Option<String>,
    occur_era: Option<String>,
  }
  // 使用salvo内置的json解析（serde-json）
  let req_bd_hot_search_r: ReqBdHotSearchR = req.parse_json().await?;
  let weibo_db_rb_conn = depot.get("weibo_db_rb_conn").map_err(|_| WeiboError::SalvoError(
    "cannot connect to the database".to_string(),
  ))?;
  let weibo_hot_search_arrs = WeiboHotSearch::weibo_hot_search_r(
    weibo_db_rb_conn, req_bd_hot_search_r.weibo_title, req_bd_hot_search_r.occur_era).await?;
  // 使用salvo内置的Json返回（serde-json）
  // Ok(Json(weibo_hot_search_arrs))
  Ok(RespBd::suc_resp(weibo_hot_search_arrs))
}

#[handler]
pub async fn hot_search_u(depot: &mut Depot) -> Result<RespBd, WeiboError> {
  let weibo_clt: &AsyncClient = depot.get("weibo_clt").unwrap();
  let weibo_db_rb_conn: &RBatis = depot.get("weibo_db_rb_conn").unwrap();
  utils::attain_ajax_hotsearch(weibo_clt, weibo_db_rb_conn).await?;
  Ok(RespBd::suc_resp("ok".to_string()))
}

#[handler]
pub async fn hot_search_d(depot: &mut Depot) -> Result<RespBd, WeiboError> {
  let weibo_db_rb_conn: &RBatis = depot.get("weibo_db_rb_conn").unwrap();
  WeiboHotSearch::weibo_hot_search_d(weibo_db_rb_conn, false, None).await?;
  Ok(RespBd::suc_resp("ok".to_string()))
}

#[handler]
pub async fn hot_timeline_r(req: &mut Request, depot: &mut Depot) -> Result<RespBd, WeiboError> {
  let mut timeline_mid: Option<String> = None;
  let mut timeline_mem_id: Option<String> = None;
  let mut timeline_mem_name: Option<String> = None;
  let mut timeline_occur_era: Option<String> = None;
  let mut pic: bool = false;
  let mut comm: bool = false;
  if let Some(req_bd_hot_timeline_r) = jzon_parse_req_bd(req).await?.as_object() {
    timeline_mid = req_bd_hot_timeline_r.get("timeline_mid").
      and_then(|val| val.as_str()).
      map(|val| val.to_string());
    timeline_mem_id = req_bd_hot_timeline_r.get("timeline_mem_id").
      and_then(|val| val.as_str()).
      map(|val| val.to_string());
    timeline_mem_name = req_bd_hot_timeline_r.get("timeline_mem_name").
      and_then(|val| val.as_str()).
      map(|val| val.to_string());
    timeline_occur_era = req_bd_hot_timeline_r.get("timeline_occur_era").
      and_then(|val| val.as_str()).
      map(|val| val.to_string());
    pic = req_bd_hot_timeline_r.get("pic").
      and_then(|val| val.as_bool()).
      ok_or_else(|| WeiboError::SalvoError("no valid pic".to_string()))?;
    comm = req_bd_hot_timeline_r.get("comm").
      and_then(|val| val.as_bool()).
      ok_or_else(|| WeiboError::SalvoError("no valid comm".to_string()))?;
  }
  let weibo_db_rb_conn = depot.get("weibo_db_rb_conn").map_err(|_| WeiboError::SalvoError(
    "cannot connect to the database".to_string(),
  ))?;
  let weibo_hot_timeline_arrs = WeiboHotTimeline::weibo_hot_timeline_r(
    weibo_db_rb_conn, timeline_mid, timeline_mem_id, timeline_mem_name, timeline_occur_era,
    pic, comm).await?;
  Ok(RespBd::suc_resp(weibo_hot_timeline_arrs))
}

#[handler]
pub async fn hot_timeline_u(req: &mut Request, depot: &mut Depot) -> Result<RespBd, WeiboError> {
  let weibo_clt: &AsyncClient = depot.get("weibo_clt").unwrap();
  let weibo_db_rb_conn: &RBatis = depot.get("weibo_db_rb_conn").unwrap();
  let mut timeline_pic = false;
  let mut timeline_comm = false;
  if let Some(req_bd_hot_timeline_u) = jzon_parse_req_bd(req).await?.as_object() {
    timeline_pic = req_bd_hot_timeline_u.get("timeline_pic").
      and_then(|val| val.as_bool()).unwrap_or_else(|| false);
    timeline_comm = req_bd_hot_timeline_u.get("timeline_comm").
      and_then(|val| val.as_bool()).unwrap_or_else(|| false);
  }
  utils::attain_ajax_hottimeline(weibo_clt, weibo_db_rb_conn, timeline_pic, timeline_comm).await?;
  Ok(RespBd::suc_resp("ok".to_string()))
}

#[handler]
pub async fn hot_timeline_d(req: &mut Request, depot: &mut Depot) -> Result<RespBd, WeiboError> {
  let mut timeline_mid: Option<String> = None;
  let mut timeline_mem_id: Option<String> = None;
  let mut timeline_mem_name: Option<String> = None;
  let mut timeline_occur_era: Option<String> = None;
  if let Some(req_bd_hot_timeline_d) = jzon_parse_req_bd(req).await?.as_object() {
    timeline_mid = req_bd_hot_timeline_d.get("timeline_mid").
      and_then(|val| val.as_str()).
      map(|val| val.to_string());
    timeline_mem_id = req_bd_hot_timeline_d.get("timeline_mem_id").
      and_then(|val| val.as_str()).
      map(|val| val.to_string());
    timeline_mem_name = req_bd_hot_timeline_d.get("timeline_mem_name").
      and_then(|val| val.as_str()).
      map(|val| val.to_string());
    timeline_occur_era = req_bd_hot_timeline_d.get("timeline_occur_era").
      and_then(|val| val.as_str()).
      map(|val| val.to_string());
  }
  let weibo_db_rb_conn: &RBatis = depot.get("weibo_db_rb_conn").unwrap();
  WeiboHotTimeline::weibo_hot_timeline_d(
    weibo_db_rb_conn, false, timeline_mid, timeline_mem_id, timeline_mem_name, timeline_occur_era).
    await?;
  Ok(RespBd::suc_resp("ok".to_string()))
}

#[handler]
pub async fn hot_timeline_comm_r(
  req: &mut Request, depot: &mut Depot) -> Result<RespBd, WeiboError> {
  let mut timeline_mid: Option<String> = None;
  let mut timeline_comm_mid: Option<String> = None;
  let mut timeline_mem_id: Option<String> = None;
  let mut timeline_mem_name: Option<String> = None;
  let mut timeline_comm_era: Option<String> = None;
  if let Some(req_bd_hot_timeline_comm_r) = jzon_parse_req_bd(req).await?.as_object() {
    timeline_mid = req_bd_hot_timeline_comm_r.get("timeline_mid").
      and_then(|val| val.as_str()).
      map(|val| val.to_string());
    timeline_comm_mid = req_bd_hot_timeline_comm_r.get("timeline_comm_mid").
      and_then(|val| val.as_str()).
      map(|val| val.to_string());
    timeline_mem_id = req_bd_hot_timeline_comm_r.get("timeline_mem_id").
      and_then(|val| val.as_str()).
      map(|val| val.to_string());
    timeline_mem_name = req_bd_hot_timeline_comm_r.get("timeline_mem_name").
      and_then(|val| val.as_str()).
      map(|val| val.to_string());
    timeline_comm_era = req_bd_hot_timeline_comm_r.get("timeline_comm_era").
      and_then(|val| val.as_str()).
      map(|val| val.to_string());
  }
  let weibo_db_rb_conn = depot.get("weibo_db_rb_conn").map_err(|_| WeiboError::SalvoError(
    "cannot connect to the database".to_string(),
  ))?;
  let weibo_hot_search_arrs = WeiboHotTimelineComm::weibo_hot_timeline_comm_r(
    weibo_db_rb_conn, timeline_mid, timeline_comm_mid, timeline_mem_id, timeline_mem_name,
    timeline_comm_era).await?;
  Ok(RespBd::suc_resp(weibo_hot_search_arrs))
}

#[handler]
pub async fn hot_timeline_comm_u(
  req: &mut Request, depot: &mut Depot) -> Result<RespBd, WeiboError> {
  let weibo_clt: &AsyncClient = depot.get("weibo_clt").unwrap();
  let weibo_db_rb_conn: &RBatis = depot.get("weibo_db_rb_conn").unwrap();

  if let Some(req_bd_hot_timeline_comm_u) = jzon_parse_req_bd(req).await?.as_object() {
    let timeline_mid = req_bd_hot_timeline_comm_u.get("timeline_mid").
      and_then(|val| val.as_str()).
      ok_or_else(|| WeiboError::SalvoError("no valid timeline_mid".to_string()))?;
    let timeline_uid = req_bd_hot_timeline_comm_u.get("timeline_uid").
      and_then(|val| val.as_str()).
      ok_or_else(|| WeiboError::SalvoError("no valid timeline_uid".to_string()))?;
    utils::attain_ajax_comments_hottimeline(
      weibo_clt, weibo_db_rb_conn, timeline_mid, timeline_uid).await?;
    Ok(RespBd::suc_resp("ok".to_string()))
  } else {
    Err(WeiboError::SalvoError("invalid search condition".to_string()))
  }
}

#[handler]
pub async fn hot_timeline_comm_d(
  req: &mut Request, depot: &mut Depot) -> Result<RespBd, WeiboError> {
  let mut timeline_mid: Option<String> = None;
  let mut timeline_comm_mid: Option<String> = None;
  let mut timeline_mem_id: Option<String> = None;
  if let Some(req_bd_hot_timeline_comm_d) = jzon_parse_req_bd(req).await?.as_object() {
    timeline_mid = req_bd_hot_timeline_comm_d.get("timeline_mid").
      and_then(|val| val.as_str()).
      map(|val| val.to_string());
    timeline_comm_mid = req_bd_hot_timeline_comm_d.get("timeline_comm_mid").
      and_then(|val| val.as_str()).
      map(|val| val.to_string());
    timeline_mem_id = req_bd_hot_timeline_comm_d.get("timeline_mem_id").
      and_then(|val| val.as_str()).
      map(|val| val.to_string());
  }
  let weibo_db_rb_conn: &RBatis = depot.get("weibo_db_rb_conn").unwrap();
  WeiboHotTimelineComm::weibo_hot_timeline_comm_d(
    weibo_db_rb_conn, false, timeline_mid, timeline_comm_mid, timeline_mem_id).await?;
  Ok(RespBd::suc_resp("ok".to_string()))
}

/// 使用jzon代替salvo内置的serde解析请求体
///
/// ## 参数
/// - `req`: salvo请求
async fn jzon_parse_req_bd(req: &mut Request) -> Result<JsonValue, WeiboError> {
  let Some(req_ctn_ilk) = req.content_type() else {
    return Err(WeiboError::SalvoError("invalid content-type".to_string()));
  };
  if req_ctn_ilk.subtype() != "json" {
    return Err(WeiboError::SalvoError("invalid content-type".to_string()));
  }
  let req_pay = req.payload().await?;
  jzon::parse(std::str::from_utf8(&req_pay).
    map_err(|flaw| WeiboError::SalvoError(flaw.to_string()))?
  ).map_err(|flaw| WeiboError::SalvoError(flaw.to_string()))
}
