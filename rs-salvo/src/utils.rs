use hifitime::efmt::consts::ISO8601_DATE;
use hifitime::prelude::Epoch;
use hifitime::prelude::Formatter;
use jzon::JsonValue as JzonValue;
use crate::dbs::WeiboHotSearch;
use crate::exceptions::WeiboError;
use crate::weibo_jzon_err;

/// 获取最新热搜并插入数据库
///
/// ## 参数
/// - `hotsearch_talk`：热搜列表，应是JSON格式
pub async fn attain_ajax_hotsearch(hotsearch_talk: &str) -> Result<(), WeiboError> {
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

  // 当前时间
  let nub_era = Epoch::now().map_err(|err| WeiboError::JzonError(err.to_string()))?;
  let nub_era = Formatter::new(nub_era, ISO8601_DATE);

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

    hot_search_arrs.push(WeiboHotSearch::weibo_hot_search_c(
      realtime_title.to_string(),
      realtime_number,
      realtime_special.to_string(),
      nub_era.to_string())
    );
  }

  WeiboHotSearch::weibo_hot_search_u(hot_search_arrs).await
}
