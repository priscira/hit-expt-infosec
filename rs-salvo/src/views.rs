use jzon::object;
use jzon::JsonValue;
use nyquest::AsyncClient;
use rbatis::RBatis;
use salvo::http::header::CONTENT_TYPE;
use salvo::http::HeaderValue;
use salvo::prelude::*;
use serde::Deserialize;
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
  #[derive(Debug, Deserialize)]
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
pub async fn hot_search_d(req: &mut Request, depot: &mut Depot) -> Result<RespBd, WeiboError> {
  let weibo_db_rb_conn: &RBatis = depot.get("weibo_db_rb_conn").unwrap();
  WeiboHotSearch::weibo_hot_search_d(weibo_db_rb_conn, false, None).await?;
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
