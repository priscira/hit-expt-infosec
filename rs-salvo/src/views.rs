use jzon::{object, JsonValue};
use nyquest::AsyncClient;
use rbatis::RBatis;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use crate::dbs::*;
use crate::exceptions::WeiboError;
use crate::utils;

#[handler]
pub async fn hello() -> String {
  "Hello World".to_string()
}

#[derive(Debug)]
struct RespBd {
  info: Option<jzon::JsonValue>,
  suc: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct RespBdHotSearchR {
  info: Option<Vec<WeiboHotSearch>>,
  suc: bool,
}

#[async_trait]
impl Writer for RespBd {
  async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.status_code(StatusCode::OK);
    res.render(jzon::stringify(self));
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

#[handler]
pub async fn hot_search_r(
  req: &mut Request, depot: &mut Depot) -> Json<RespBdHotSearchR> {
  #[derive(Debug, Deserialize)]
  struct ReqBdHotSearchR {
    weibo_title: Option<String>,
    occur_era: Option<String>,
  }
  let Ok(req_bd_hot_search_r) = req.parse_json::<ReqBdHotSearchR>().await
  else {
    return Json(RespBdHotSearchR {
      info: None,
      suc: false,
    });
  };
  let Ok(weibo_db_rb_conn) = depot.get("weibo_db_rb_conn").map_err(|_| WeiboError::SalvoError(
    "cannot connect to the database".to_string(),
  )) else {
    return Json(RespBdHotSearchR {
      info: None,
      suc: false,
    })
  };
  if let Ok(weibo_hot_search_arrs) = WeiboHotSearch::weibo_hot_search_r(
    weibo_db_rb_conn, req_bd_hot_search_r.weibo_title, req_bd_hot_search_r.occur_era).await {
    Json(RespBdHotSearchR {
      info: Some(weibo_hot_search_arrs),
      suc: true,
    })
  } else {
    Json(RespBdHotSearchR {
      info: None,
      suc: false,
    })
  }
}

#[derive(Debug, Serialize, Deserialize)]
struct RespBdHotSearchU {
  info: Option<String>,
  suc: bool,
}

#[handler]
pub async fn hot_search_u(depot: &mut Depot) -> Json<RespBdHotSearchU> {
  let weibo_clt: &AsyncClient = depot.get("weibo_clt").unwrap();
  let weibo_db_rb_conn: &RBatis = depot.get("weibo_db_rb_conn").unwrap();
  if let Err(weibo_flaw) = utils::attain_ajax_hotsearch(weibo_clt, weibo_db_rb_conn).await {
    Json(RespBdHotSearchU {
      info: Some(weibo_flaw.to_string()),
      suc: false,
    })
  } else {
    Json(RespBdHotSearchU {
      info: Some("success".to_string()),
      suc: true,
    })
  }
}

#[handler]
pub async fn hot_search_d(depot: &mut Depot) -> String {
  let custom_data: &AsyncClient = depot.get("weibo_clt").unwrap();
  let weibo_db_rb_conn: &RBatis = depot.get("weibo_db_rb_conn").unwrap();
  println!("####");
  "Hello World".to_string()
}
