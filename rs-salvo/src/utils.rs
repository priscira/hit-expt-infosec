use hifitime::efmt::consts::ISO8601_DATE;
use hifitime::prelude::Epoch;
use hifitime::prelude::Formatter;
use jzon::JsonValue as JzonValue;
use crate::dbs::WeiboHotSearch;
use crate::dbs::WeiboHotTimeline;
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
  let nub_era = Epoch::now()?;
  let nub_era = Formatter::new(nub_era, ISO8601_DATE);

  for hot_search_realtime_arri in hot_search_realtime_arrs.iter() {
    // 判断是否是广告
    if let Some(1) = hot_search_realtime_arri.get("is_ad").and_then(|val| val.as_u8()) {
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


pub async fn attain_ajax_hottimeline(hottimeline_talk: &str) -> Result<(), WeiboError> {
  let mut hot_timeline_arrs = vec![];
  println!("开始JSON解析");
  let hot_timeline_jquin = jzon::parse(&hottimeline_talk)?;
  println!("结束JSON解析");
  let hot_timeline_statuses = hot_timeline_jquin.get("statuses")
    .ok_or_else(|| weibo_jzon_err!("/ajax/feed/hottimeline no field statuses"))?;
  let hot_timeline_status_arrs: &Vec<JzonValue> = hot_timeline_statuses.as_array()
    .ok_or_else(
      || weibo_jzon_err!("/ajax/feed/hottimeline statuses is not array"))?;

  for hot_timeline_status_arri in hot_timeline_status_arrs.iter() {
    let Some(timeline_mid) = hot_timeline_status_arri.get("mid").and_then(|val| val.as_str()) else {
      continue
    };

    let Some(timeline_mblogid) = hot_timeline_status_arri.get("mblogid").and_then(|val| val.as_str()
    ) else {
      continue
    };

    let Some(timeline_mem) = hot_timeline_status_arri.get("user").and_then(|val| val.as_object())
    else {
      continue;
    };

    let timeline_text = hot_timeline_status_arri.get("text_raw").and_then(
      |val| val.as_str()).unwrap_or("");
    let timeline_mem_id = timeline_mem.get("idstr").and_then(|val| val.as_str()).unwrap_or("");
    let timeline_mem_name = timeline_mem.get("screen_name").and_then(|val| val.as_str()
    ).unwrap_or("");

    let timeline_era = match hot_timeline_status_arri.get("created_at").and_then(|val| val.as_str()
    ) {
      Some(era_talk) => {
        println!("时间: {:?}", &era_talk.replace("+8000 ", ""));
        let occ_era = Epoch::from_format_str(
          &era_talk.replace("+0800 ", ""), "%a %b %d %H:%M:%S %Y")?;
        Formatter::new(occ_era, ISO8601_DATE).to_string()
      }
      None => {
        let nub_era = Epoch::now()?;
        Formatter::new(nub_era, ISO8601_DATE).to_string()
      }
    };

    hot_timeline_arrs.push(WeiboHotTimeline::weibo_hot_timeline_c(
      timeline_mid.to_string(),
      timeline_mblogid.to_string(),
      timeline_text.to_string(),
      timeline_mem_id.to_string(),
      timeline_mem_name.to_string(),
      timeline_era,
    ));
  }
  println!("=========");
  for hot_timeline_arri in hot_timeline_arrs.iter() {
    println!("{:?}", hot_timeline_arri);
    println!("=========");
  }

  WeiboHotTimeline::weibo_hot_timeline_u(hot_timeline_arrs).await
}
