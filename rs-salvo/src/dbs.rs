use rbatis::RBatis;
use serde::Deserialize;
use serde::Serialize;
use crate::exceptions::WeiboError;

/// 微博热搜
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WeiboHotSearch {
  pub id: Option<usize>,
  // 热搜标题
  pub title: String,
  // 热搜热度
  pub number: u32,
  // 热搜是否为特殊热搜，比如“热”、“新”
  pub special: String,
  // 热搜出现的时间，格式为YYYY-MM-DD
  pub occur_era: String,
}
rbatis::crud!(WeiboHotSearch {}, "weibo_hot_search");

impl WeiboHotSearch {
  /// 创建一个微博热搜WeiboHotSearch对象
  ///
  /// ## 参数
  /// - `realtime_title`: 热搜标题
  /// - `realtime_number`: 热搜热度
  /// - `realtime_special`: 热搜是否为特殊热搜，比如“热”、“新”
  /// - `occur_era`: 热搜出现的时间，格式为YYYY-MM-DD
  pub fn weibo_hot_search_c(
    realtime_title: String, realtime_number: u32, realtime_special: String, occur_era: String,
  ) -> Self {
    Self {
      id: None,
      title: realtime_title,
      number: realtime_number,
      special: realtime_special,
      occur_era,
    }
  }

  /// 获取微博热搜WeiboHotSearch对象
  ///
  /// ## 参数
  /// - `weibo_db_rb_conn`：rbatis数据库连接
  /// - `weibo_title`: 热搜标题，可选
  /// - `occur_era`: 热搜出现的年月日，格式YYYY-MM-DD，可选
  ///
  /// ## 返回
  /// 成功则返回符合查询条件的微博热搜数据
  pub async fn weibo_hot_search_r(weibo_db_rb_conn: &RBatis, weibo_title: Option<String>,
                                  occur_era: Option<String>) -> Result<Vec<Self>, WeiboError> {
    let mut weibo_hot_search_r_qry = rbs::value! {};
    if let Some(weibo_title) = weibo_title {
      weibo_hot_search_r_qry.insert(rbs::value!("title"), rbs::value!(weibo_title));
    }
    if let Some(occur_era) = occur_era {
      weibo_hot_search_r_qry.insert(rbs::value!("occur_era"), rbs::value!(occur_era));
    }

    Self::select_by_map(weibo_db_rb_conn, weibo_hot_search_r_qry).await.map_err(|flaw| {
      WeiboError::RbatisError(flaw.to_string())
    })
  }

  /// 更新微博热搜WeiboHotSearch数据，如果有当天的同名的热搜，那么不做处理；否则直接插入。
  ///
  /// ## 参数
  /// - `weibo_db_rb_conn`：rbatis数据库连接
  /// - `hot_search_arrs`: 新的微博热搜数据
  pub async fn weibo_hot_search_u(
    weibo_db_rb_conn: &RBatis, hot_search_arrs: Vec<WeiboHotSearch>) -> Result<(), WeiboError> {
    if hot_search_arrs.is_empty() {
      return Ok(());
    }

    let mut weibo_hot_search_ques = vec![];
    let mut weibo_hot_search_pars = vec![];
    for hot_search_arri in hot_search_arrs.iter() {
      weibo_hot_search_ques.push("(?, ?, ?, ?)");
      weibo_hot_search_pars.push(rbs::value!(hot_search_arri.title.clone()));
      weibo_hot_search_pars.push(rbs::value!(hot_search_arri.number));
      weibo_hot_search_pars.push(rbs::value!(hot_search_arri.special.clone()));
      weibo_hot_search_pars.push(rbs::value!(hot_search_arri.occur_era.clone()));
    }

    let weibo_hot_search_sent = format!(
      "insert or ignore into weibo_hot_search (title, number, special, occur_era) values {}",
      weibo_hot_search_ques.join(", "));

    weibo_db_rb_conn.exec(&weibo_hot_search_sent, weibo_hot_search_pars).await.map(|_| ()
    ).map_err(|flaw| {
      WeiboError::RbatisError(flaw.to_string())
    })
  }

  /// 删除微博热搜WeiboHotSearch数据。
  ///
  /// ## 参数
  /// - `weibo_db_rb_conn`：rbatis数据库连接
  /// - `no_sieve`: 无筛选条件，删除全部数据时的保证参数
  /// - `occur_era`: 热搜出现的年月日，格式YYYY-MM-DD，可选
  pub async fn weibo_hot_search_d(weibo_db_rb_conn: &RBatis, no_sieve: bool,
                                  occur_era: Option<String>) -> Result<(), WeiboError> {
    let mut weibo_hot_search_d_qry = rbs::value! {};

    if let Some(occur_era) = occur_era {
      weibo_hot_search_d_qry.insert(rbs::value!("occur_era"), rbs::value!(occur_era));
    } else if !no_sieve {
      return Err(WeiboError::RbatisError("delete all the data from the database, \
                                         but no_sieve guarantee isn't provided.".to_string()));
    }

    Self::delete_by_map(weibo_db_rb_conn, weibo_hot_search_d_qry).await.map(|_| ()).map_err(
      |flaw| {
        WeiboError::RbatisError(flaw.to_string())
      }
    )
  }
}

/// 微博热门推荐
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WeiboHotTimeline {
  pub id: Option<usize>,
  // 热门推荐的mid
  pub mid: String,
  // 热门推荐的mblogid
  pub mblogid: String,
  // 热门推荐的内容
  pub text: String,
  // 热门推荐的发布者的编号
  pub mem_id: String,
  // 热门推荐的发布者的名称
  pub mem_name: String,
  // 热门推荐出现的时间，格式为YYYY-MM-DD
  pub occur_era: String,
}
rbatis::crud!(WeiboHotTimeline {}, "weibo_hot_timeline");

impl WeiboHotTimeline {
  /// 创建一个微博热门推荐WeiboHotTimeline对象
  ///
  /// ## 参数
  /// - `timeline_mid`: 热门推荐的mid
  /// - `timeline_mblogid`: 热门推荐的mblogid
  /// - `timeline_text`: 热门推荐的内容
  /// - `timeline_mem_id`: 热门推荐的发布者的编号
  /// - `timeline_mem_name`: 热门推荐的发布者的名称
  /// - `timeline_occur_era`: 热门推荐出现的时间，格式为YYYY-MM-DD
  pub fn weibo_hot_timeline_c(timeline_mid: String, timeline_mblogid: String, timeline_text: String,
                              timeline_mem_id: String, timeline_mem_name: String,
                              timeline_occur_era: String) -> Self {
    Self {
      id: None,
      mid: timeline_mid,
      mblogid: timeline_mblogid,
      text: timeline_text,
      mem_id: timeline_mem_id,
      mem_name: timeline_mem_name,
      occur_era: timeline_occur_era,
    }
  }

  /// 获取微博热门推荐WeiboHotTimeline对象
  ///
  /// ## 参数
  /// - `weibo_db_rb_conn`：rbatis数据库连接
  /// - `timeline_mid`: 热门推荐的mid，可选
  /// - `timeline_mem_id`: 热门推荐的发布者的编号，可选
  /// - `timeline_mem_name`: 热门推荐的发布者的名称，可选
  /// - `timeline_occur_era`: 热门推荐出现的时间，格式YYYY-MM-DD，可选
  ///
  /// ## 返回
  /// 成功则返回符合查询条件的微博热门推荐数据
  pub async fn weibo_hot_timeline_r(
    weibo_db_rb_conn: &RBatis, timeline_mid: Option<String>, timeline_mem_id: Option<String>,
    timeline_mem_name: Option<String>, timeline_occur_era: Option<String>)
    -> Result<Vec<Self>, WeiboError> {
    let mut weibo_hot_timeline_r_qry = rbs::value! {};
    if let Some(timeline_mid) = timeline_mid {
      weibo_hot_timeline_r_qry.insert(rbs::value!("mid"), rbs::value!(timeline_mid));
    }
    if let Some(timeline_mem_id) = timeline_mem_id {
      weibo_hot_timeline_r_qry.insert(rbs::value!("mem_id"), rbs::value!(timeline_mem_id));
    }
    if let Some(timeline_mem_name) = timeline_mem_name {
      weibo_hot_timeline_r_qry.insert(rbs::value!("mem_name"), rbs::value!(timeline_mem_name));
    }
    if let Some(timeline_occur_era) = timeline_occur_era {
      weibo_hot_timeline_r_qry.insert(rbs::value!("occur_era"), rbs::value!(timeline_occur_era));
    }

    Self::select_by_map(weibo_db_rb_conn, weibo_hot_timeline_r_qry).await.map_err(|flaw| {
      WeiboError::RbatisError(flaw.to_string())
    })
  }

  /// 更新微博热门推荐WeiboHotTimeline数据，如果有相同的mid则更新；否则直接插入。
  ///
  /// ## 参数
  /// - `weibo_db_rb_conn`：rbatis数据库连接
  /// - `hot_timeline_arrs`: 新的微博热门推荐数据
  pub async fn weibo_hot_timeline_u(
    weibo_db_rb_conn: &RBatis, hot_timeline_arrs: Vec<Self>) -> Result<(), WeiboError> {
    if hot_timeline_arrs.is_empty() {
      return Ok(());
    }

    let mut weibo_hot_timeline_ques = vec![];
    let mut weibo_hot_timeline_pars = vec![];
    for hot_timeline_arri in hot_timeline_arrs.iter() {
      weibo_hot_timeline_ques.push("(?, ?, ?, ?, ?, ?)");
      weibo_hot_timeline_pars.push(rbs::value!(hot_timeline_arri.mid.clone()));
      weibo_hot_timeline_pars.push(rbs::value!(hot_timeline_arri.mblogid.clone()));
      weibo_hot_timeline_pars.push(rbs::value!(hot_timeline_arri.text.clone()));
      weibo_hot_timeline_pars.push(rbs::value!(hot_timeline_arri.mem_id.clone()));
      weibo_hot_timeline_pars.push(rbs::value!(hot_timeline_arri.mem_name.clone()));
      weibo_hot_timeline_pars.push(rbs::value!(hot_timeline_arri.occur_era.clone()));
    }

    let weibo_hot_search_sent = format!(
      "insert into weibo_hot_timeline (mid, mblogid, text, mem_id, mem_name, occur_era) \
       values {} \
       on conflict(mid) do update set \
         mblogid = excluded.mblogid, \
         text = excluded.text, \
         mem_name = excluded.mem_name, \
         occur_era = excluded.occur_era",
      weibo_hot_timeline_ques.join(", "));

    weibo_db_rb_conn.exec(&weibo_hot_search_sent, weibo_hot_timeline_pars).await.map(|_| ()
    ).map_err(|flaw| {
      WeiboError::RbatisError(flaw.to_string())
    })
  }

  /// 删除微博热门推荐WeiboHotTimeline数据。
  ///
  /// ## 参数
  /// - `weibo_db_rb_conn`：rbatis数据库连接
  /// - `no_sieve`: 无筛选条件，删除全部数据时的保证参数
  /// - `timeline_mid`: 热门推荐的mid，可选
  /// - `timeline_mem_id`: 热门推荐的发布者的编号，可选
  /// - `timeline_mem_name`: 热门推荐的发布者的名称，可选
  /// - `timeline_occur_era`: 热门推荐出现的年月日，格式YYYY-MM-DD，可选
  pub async fn weibo_hot_timeline_d(
    weibo_db_rb_conn: &RBatis,
    no_sieve: bool, timeline_mid: Option<String>, timeline_mem_id: Option<String>,
    timeline_mem_name: Option<String>, timeline_occur_era: Option<String>)
    -> Result<(), WeiboError> {
    let mut weibo_hot_timeline_d_qry = rbs::value! {};

    if let Some(timeline_mid) = timeline_mid {
      weibo_hot_timeline_d_qry.insert(rbs::value!("mid"), rbs::value!(timeline_mid));
    }
    if let Some(timeline_mem_id) = timeline_mem_id {
      weibo_hot_timeline_d_qry.insert(rbs::value!("mem_id"), rbs::value!(timeline_mem_id));
    }
    if let Some(timeline_mem_name) = timeline_mem_name {
      weibo_hot_timeline_d_qry.insert(rbs::value!("mem_name"), rbs::value!(timeline_mem_name));
    }
    if let Some(timeline_occur_era) = timeline_occur_era {
      weibo_hot_timeline_d_qry.insert(rbs::value!("occur_era"), rbs::value!(timeline_occur_era));
    }
    if weibo_hot_timeline_d_qry.is_empty() && !no_sieve {
      return Err(WeiboError::RbatisError("delete all the data from the database, \
                                         but no_sieve guarantee isn't provided.".to_string()));
    }

    Self::delete_by_map(weibo_db_rb_conn, weibo_hot_timeline_d_qry).await.map(|_| ()).map_err(
      |flaw| {
        WeiboError::RbatisError(flaw.to_string())
      }
    )
  }
}

/// 微博热门推荐的图片
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WeiboHotTimelinePic {
  pub id: Option<usize>,
  // 图片所属的热门推荐的mid
  pub mid: String,
  // 图片id
  pub pic_id: String,
  // 图片url
  pub pic_url: String,
}
rbatis::crud!(WeiboHotTimelinePic {}, "weibo_hot_timeline_pic");

impl WeiboHotTimelinePic {
  /// 创建一个微博热门推荐图片WeiboHotTimelinePic对象
  ///
  /// ## 参数
  /// - `timeline_mid`: 图片所属的热门推荐的mid
  /// - `timeline_pic_id`: 图片id
  /// - `timeline_pic_url`: 图片url
  pub fn weibo_hot_timeline_pic_c(timeline_mid: String, timeline_pic_id: String,
                                  timeline_pic_url: String) -> Self {
    Self {
      id: None,
      mid: timeline_mid,
      pic_id: timeline_pic_id,
      pic_url: timeline_pic_url,
    }
  }

  /// 获取微博热门推荐图片WeiboHotTimelinePic对象
  ///
  /// ## 参数
  /// - `weibo_db_rb_conn`：rbatis数据库连接
  /// - `timeline_mid`: 热门推荐的mid，可选
  ///
  /// ## 返回
  /// 成功则返回符合查询条件的微博热门推荐图片数据
  pub async fn weibo_hot_timeline_pic_r(
    weibo_db_rb_conn: &RBatis, timeline_mid: Option<String>) -> Result<Vec<Self>, WeiboError> {
    let mut weibo_hot_timeline_pic_r_qry = rbs::value! {};
    if let Some(timeline_mid) = timeline_mid {
      weibo_hot_timeline_pic_r_qry.insert(rbs::value!("mid"), rbs::value!(timeline_mid));
    }

    Self::select_by_map(weibo_db_rb_conn, weibo_hot_timeline_pic_r_qry).await.map_err(|flaw| {
      WeiboError::RbatisError(flaw.to_string())
    })
  }

  /// 更新微博热门推荐图片WeiboHotTimelinePic数据
  ///
  /// ## 参数
  /// - `weibo_db_rb_conn`：rbatis数据库连接
  /// - `hot_timeline_pic_arrs`: 新的微博热门推荐图片数据
  pub async fn weibo_hot_timeline_pic_u(
    weibo_db_rb_conn: &RBatis, hot_timeline_pic_arrs: Vec<Self>) -> Result<(), WeiboError> {
    if hot_timeline_pic_arrs.is_empty() {
      return Ok(());
    }

    Self::insert_batch(weibo_db_rb_conn, &hot_timeline_pic_arrs, 10).await.map(|_| ()).map_err(
      |flaw| {
        WeiboError::RbatisError(flaw.to_string())
      }
    )
  }

  /// 删除微博热门推荐图片WeiboHotTimelinePic数据
  ///
  /// ## 参数
  /// - `weibo_db_rb_conn`：rbatis数据库连接
  /// - `no_sieve`: 无筛选条件，删除全部数据时的保证参数
  /// - `timeline_mid`: 热门推荐的mid，可选
  pub async fn weibo_hot_timeline_pic_d(weibo_db_rb_conn: &RBatis, no_sieve: bool,
                                        timeline_mid: Option<String>) -> Result<(), WeiboError> {
    let mut weibo_hot_timeline_pic_d_qry = rbs::value! {};

    if let Some(timeline_mid) = timeline_mid {
      weibo_hot_timeline_pic_d_qry.insert(rbs::value!("mid"), rbs::value!(timeline_mid));
    } else if !no_sieve {
      return Err(WeiboError::RbatisError("delete all the data from the database, \
                                         but no_sieve guarantee isn't provided.".to_string()));
    }

    Self::delete_by_map(weibo_db_rb_conn, weibo_hot_timeline_pic_d_qry).await.map(|_| ()).map_err(
      |flaw| {
        WeiboError::RbatisError(flaw.to_string())
      }
    )
  }
}

/// 微博热门推荐的评论
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WeiboHotTimelineComm {
  pub id: Option<usize>,
  pub mid: String,
  pub comm_mid: String,
  pub text: String,
  pub mem_id: String,
  pub mem_name: String,
  pub comm_era: String,
  // 是否是评论回复
  pub reply: bool,
  // 如果是评论回复，存储其根评论的id
  pub senior_id: String,
}
rbatis::crud!(WeiboHotTimelineComm {}, "weibo_hot_timeline_comm");

impl WeiboHotTimelineComm {
  /// 创建一个微博热门推荐评论WeiboHotTimelineComm对象
  ///
  /// ## 参数
  /// - `timeline_mid`: 热门推荐的mid
  /// - `timeline_comm_mid`: 评论的mid
  /// - `timeline_text`: 评论内容
  /// - `timeline_mem_id`: 评论用户id
  /// - `timeline_mem_name`: 评论用户名
  /// - `timeline_comm_era`: 评论时间
  /// - `timeline_reply`: 是否是评论回复
  /// - `timeline_senior_id`: 如果是评论回复，存储其根评论的id
  pub fn weibo_hot_timeline_comm_c(timeline_mid: String, timeline_comm_mid: String,
                                   timeline_text: String, timeline_mem_id: String,
                                   timeline_mem_name: String, timeline_comm_era: String,
                                   timeline_reply: bool, timeline_senior_id: String) -> Self {
    Self {
      id: None,
      mid: timeline_mid,
      comm_mid: timeline_comm_mid,
      text: timeline_text,
      mem_id: timeline_mem_id,
      mem_name: timeline_mem_name,
      comm_era: timeline_comm_era,
      reply: timeline_reply,
      senior_id: timeline_senior_id,
    }
  }

  /// 获取微博热门推荐评论WeiboHotTimelineComm对象
  ///
  /// ## 参数
  /// - `weibo_db_rb_conn`：rbatis数据库连接
  /// - `timeline_mid`: 热门推荐的mid，可选
  /// - `timeline_comm_mid`: 评论的mid，可选
  /// - `timeline_mem_id`: 评论用户id，可选
  /// - `timeline_mem_name`: 评论用户名，可选
  /// - `timeline_comm_era`: 评论时间，可选
  ///
  /// ## 返回
  /// 成功则返回符合查询条件的微博热门推荐评论数据
  pub async fn weibo_hot_timeline_comm_r(
    weibo_db_rb_conn: &RBatis,
    timeline_mid: Option<String>, timeline_comm_mid: Option<String>,
    timeline_mem_id: Option<String>, timeline_mem_name: Option<String>,
    timeline_comm_era: Option<String>) -> Result<Vec<Self>, WeiboError> {
    let mut weibo_hot_timeline_comm_r_qry = rbs::value! {};
    if let Some(timeline_mid) = timeline_mid {
      weibo_hot_timeline_comm_r_qry.insert(rbs::value!("mid"), rbs::value!(timeline_mid));
    }
    if let Some(timeline_comm_mid) = timeline_comm_mid {
      weibo_hot_timeline_comm_r_qry.insert(rbs::value!("comm_mid"), rbs::value!(timeline_comm_mid));
    }
    if let Some(timeline_mem_id) = timeline_mem_id {
      weibo_hot_timeline_comm_r_qry.insert(rbs::value!("mem_id"), rbs::value!(timeline_mem_id));
    } else if let Some(timeline_mem_name) = timeline_mem_name {
      weibo_hot_timeline_comm_r_qry.insert(
        rbs::value!("mem_name"), rbs::value!(timeline_mem_name));
    }
    if let Some(timeline_comm_era) = timeline_comm_era {
      weibo_hot_timeline_comm_r_qry.insert(rbs::value!("comm_era"), rbs::value!(timeline_comm_era));
    }

    Self::select_by_map(weibo_db_rb_conn, weibo_hot_timeline_comm_r_qry).await.map_err(|flaw| {
      WeiboError::RbatisError(flaw.to_string())
    })
  }

  /// 更新微博热门推荐评论WeiboHotTimelineComm数据
  ///
  /// ## 参数
  /// - `weibo_db_rb_conn`：rbatis数据库连接
  /// - `hot_timeline_comm_arrs`: 新的微博热门推荐评论数据
  pub async fn weibo_hot_timeline_comm_u(
    weibo_db_rb_conn: &RBatis, hot_timeline_comm_arrs: Vec<Self>) -> Result<(), WeiboError> {
    if hot_timeline_comm_arrs.is_empty() {
      return Ok(());
    }

    Self::insert_batch(weibo_db_rb_conn, &hot_timeline_comm_arrs, 10).await.map(|_| ()).map_err(
      |flaw| {
        WeiboError::RbatisError(flaw.to_string())
      }
    )
  }

  /// 删除微博热门推荐评论WeiboHotTimelineComm数据
  ///
  /// ## 参数
  /// - `weibo_db_rb_conn`：rbatis数据库连接
  /// - `no_sieve`: 无筛选条件，删除全部数据时的保证参数
  /// - `timeline_mid`: 热门推荐的mid，可选
  /// - `timeline_comm_mid`: 评论的mid，可选
  /// - `timeline_mem_id`: 评论用户id，可选
  pub async fn weibo_hot_timeline_comm_d(
    weibo_db_rb_conn: &RBatis,
    no_sieve: bool, timeline_mid: Option<String>, timeline_comm_mid: Option<String>,
    timeline_mem_id: Option<String>) -> Result<(), WeiboError> {
    let mut weibo_hot_timeline_comm_d_qry = rbs::value! {};

    if let Some(timeline_mid) = timeline_mid {
      weibo_hot_timeline_comm_d_qry.insert(rbs::value!("mid"), rbs::value!(timeline_mid));
    }
    if let Some(timeline_comm_mid) = timeline_comm_mid {
      weibo_hot_timeline_comm_d_qry.insert(rbs::value!("comm_mid"), rbs::value!(timeline_comm_mid));
    }
    if let Some(timeline_mem_id) = timeline_mem_id {
      weibo_hot_timeline_comm_d_qry.insert(rbs::value!("mem_id"), rbs::value!(timeline_mem_id));
    }
    if weibo_hot_timeline_comm_d_qry.is_empty() && !no_sieve {
      return Err(WeiboError::RbatisError("delete all the data from the database, \
                                         but no_sieve guarantee isn't provided.".to_string()));
    }

    Self::delete_by_map(weibo_db_rb_conn, weibo_hot_timeline_comm_d_qry).await.map(|_| ()).map_err(
      |flaw| {
        WeiboError::RbatisError(flaw.to_string())
      }
    )
  }
}
