use jzon::JsonValue as JzonValue;
use njord::keys::AutoIncrementPrimaryKey;
use njord::sqlite;
use crate::dbs::WeiboHotSearch;
use crate::exceptions::WeiboError;
use crate::prefs::WEIBO_DB_PTH;
use crate::weibo_jzon_err;

pub fn attain_ajax_hotsearch(hotsearch_talk: &str) -> Result<(), WeiboError> {
  let mut hot_search_arrs = vec![];
  let hot_search_jquin = jzon::parse(&hotsearch_talk)?;
  let hot_search_data = hot_search_jquin.get("data")
    .ok_or_else(|| weibo_jzon_err!("/ajax/side/hotSearch no field data"))?;
  let hot_search_real_time = hot_search_data.get("realtime")
    .ok_or_else(
      || weibo_jzon_err!("/ajax/side/hotSearch no field data.realtime"))?;
  let hot_search_realtime_arrs: &Vec<JzonValue> = hot_search_real_time.as_array()
    .ok_or_else(
      || weibo_jzon_err!("/ajax/side/hotSearch data.realtime is not array"))?;
  for hot_search_realtime_arri in hot_search_realtime_arrs.iter() {
    // 判断是否是广告
    if let Some(1) = hot_search_realtime_arri.get("is_ad").and_then(|v| v.as_u8()) {
      continue;
    }

    let Some(realtime_title) = hot_search_realtime_arri.get("word").and_then(|val| val.as_str())
    else {
      continue;
    };
    let realtime_number = hot_search_realtime_arri.get("num")
      .and_then(|val| val.as_u32())
      .unwrap_or(0);
    let realtime_special = hot_search_realtime_arri.get("label_name")
      .and_then(|val| val.as_str())
      .unwrap_or("");

    hot_search_arrs.push(WeiboHotSearch {
      id: AutoIncrementPrimaryKey::default(),
      title: realtime_title.to_string(),
      number: realtime_number,
      special: realtime_special.to_string(),
      occur_time: "2025-11-19".to_string(),
    })
  }

  let weibo_db_pth = std::path::Path::new(WEIBO_DB_PTH);
  let weibo_db_conn = sqlite::open(weibo_db_pth).map_err(|err| {
    WeiboError::NjordError(err.to_string())
  })?;
  sqlite::insert(&weibo_db_conn, hot_search_arrs).map_err(|err| {
    WeiboError::NjordError(err.to_string())
  })?;

  Ok(())
}
