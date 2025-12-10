use hifitime::efmt::consts::ISO8601_DATE;
use hifitime::prelude::Epoch;
use hifitime::prelude::Formatter;
use jzon::JsonValue;
use nyquest::AsyncClient;
use crate::dbs::WeiboHotSearch;
use crate::dbs::WeiboHotTimeline;
use crate::dbs::WeiboHotTimelinePic;
use crate::exceptions::WeiboError;
use crate::weibo;
use crate::weibo_jzon_err;

/// 获取最新热搜并插入数据库
///
/// ## 参数
/// - `weibo_clt`：nyquest异步HTTP客户端
pub async fn attain_ajax_hotsearch(weibo_clt: &AsyncClient) -> Result<(), WeiboError> {
  // 热搜列表，应是JSON格式
  let hotsearch_talk: String = weibo::gain_side_hotsearch(&weibo_clt).await?;

  let mut hot_search_arrs = vec![];
  let hot_search_jquin = jzon::parse(&hotsearch_talk)?;
  let hot_search_data = hot_search_jquin.get("data")
    .ok_or_else(|| weibo_jzon_err!("/ajax/side/hotSearch no field data"))?;
  let hot_search_real_time = hot_search_data.get("realtime")
    .ok_or_else(
      || weibo_jzon_err!("/ajax/side/hotSearch no field data.realtime"))?;
  let hot_search_realtime_arrs: &Vec<JsonValue> = hot_search_real_time.as_array()
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

/// 获取最新热门推荐并插入数据库
///
/// ## 参数
/// - `weibo_clt`：nyquest异步HTTP客户端
/// - `pic`：是否需要爬取图片
pub async fn attain_ajax_hottimeline(weibo_clt: &AsyncClient, pic: bool) -> Result<(), WeiboError> {
  // 热门推荐列表，应是JSON格式
  let hottimeline_talk: String = weibo::gain_side_hotsearch(&weibo_clt).await?;

  let mut hot_timeline_arrs = vec![];
  let hot_timeline_jquin = jzon::parse(&hottimeline_talk)?;
  let hot_timeline_statuses = hot_timeline_jquin.get("statuses")
    .ok_or_else(|| weibo_jzon_err!("/ajax/feed/hottimeline no field statuses"))?;
  let hot_timeline_status_arrs: &Vec<JsonValue> = hot_timeline_statuses.as_array()
    .ok_or_else(
      || weibo_jzon_err!("/ajax/feed/hottimeline statuses is not array"))?;

  for hot_timeline_status_arri in hot_timeline_status_arrs.iter() {
    // 判断是否是广告
    if let Some(true) = hot_timeline_status_arri.get("isAd").and_then(|val| val.as_bool()) {
      println!("== 广告: {}", hot_timeline_status_arri);
      continue;
    }

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

    if pic {
      let timeline_pic_infos: Option<&JsonValue> = hot_timeline_status_arri.get("pic_infos");
      let timeline_mix_media_infos: Option<&JsonValue> =
        hot_timeline_status_arri.get("mix_media_info");
    }

    let timeline_text = hot_timeline_status_arri.get("text_raw").and_then(
      |val| val.as_str()).unwrap_or("");
    let timeline_mem_id = timeline_mem.get("idstr").and_then(|val| val.as_str()).unwrap_or("");
    let timeline_mem_name = timeline_mem.get("screen_name").and_then(|val| val.as_str()
    ).unwrap_or("");

    let timeline_era = hot_timeline_status_arri.get("created_at").and_then(|v| v.as_str()).
      and_then(|era_talk| {
        Epoch::from_format_str(&era_talk.replace("+0800 ", ""), "%a %b %d %H:%M:%S %Y").
          map(|era_val| Formatter::new(era_val, ISO8601_DATE).to_string()).ok()
      }).
      or_else(|| {
        Epoch::now().map(|era_val| Formatter::new(era_val, ISO8601_DATE).to_string()).ok()
      });
    let timeline_era = if let Some(timeline_era) = timeline_era {
      timeline_era
    } else {
      continue;
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

  WeiboHotTimeline::weibo_hot_timeline_u(hot_timeline_arrs).await
}

pub async fn attain_sinaimg_hot_timeline(weibo_clt: &AsyncClient,
                                         timeline_mid: &str,
                                         timeline_pic_infos: Option<&JsonValue>,
                                         timeline_mix_media_infos: Option<&JsonValue>,
) -> Result<Vec<WeiboHotTimelinePic>, WeiboError> {
  // 从pic_infos中获取图片信息，包括url和pic_id
  let anly_pic_info_large = |pic_info_large: Option<&JsonValue>| {
    pic_info_large.and_then(|pic_info_large| pic_info_large.as_object()).
      and_then(|pic_info_large| {
        let pic_info_large_url = pic_info_large.get("url").and_then(|v| v.as_str())?;
        let pic_info_large_pic_id = pic_info_large.get("pic_id").and_then(|v| v.as_str())?;
        Some((pic_info_large_url.to_string(), pic_info_large_pic_id.to_string()))
      })
  };

  let mut hot_timeline_pic_arrs: Vec<WeiboHotTimelinePic> = Vec::new();

  if let Some(timeline_pic_infos) = timeline_pic_infos.and_then(|val| val.as_object()) {
    // 直接从pic_infos中获取图片信息
    for (_, timeline_pic_infoi) in timeline_pic_infos.iter() {
      if let Some((pic_url, pic_id)) = anly_pic_info_large(timeline_pic_infoi.get("large")) {
        hot_timeline_pic_arrs.push(WeiboHotTimelinePic::weibo_hot_timeline_pic_c(
          timeline_mid.to_string(), pic_id, pic_url,
        ));
      }
    }
  } else if let Some(timeline_mix_media_infos) = timeline_mix_media_infos {
    // 有视频，从mix_media_infos中获取图片信息
    if let Some(mix_media_info_itms) = timeline_mix_media_infos.get("items").
      and_then(|val| val.as_array()) {
      for mix_media_info_itmi in mix_media_info_itms {
        if let Some("pic") = mix_media_info_itmi.get("type").and_then(|val| val.as_str()) {
          if let Some((pic_url, pic_id)) = anly_pic_info_large(mix_media_info_itmi.get("data").
            and_then(|val| val.as_object()).
            map(|mix_media_info_large| mix_media_info_large.get("large")).
            flatten()
          ) {
            hot_timeline_pic_arrs.push(WeiboHotTimelinePic::weibo_hot_timeline_pic_c(
              timeline_mid.to_string(), pic_id, pic_url,
            ));
          }
        }
      }
    }
  }

  Ok(hot_timeline_pic_arrs)
}
