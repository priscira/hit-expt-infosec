use rbatis::RBatis;
use rbdc_sqlite::SqliteDriver;
use serde::Deserialize;
use serde::Serialize;
use crate::exceptions::WeiboError;
use crate::prefs::WEIBO_DB_PTH;


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
  /// - `weibo_title`: 热搜标题，可选
  /// - `occur_era`: 热搜出现的年月日，格式YYYY-MM-DD，可选
  ///
  /// ## 返回
  /// 成功则返回符合查询条件的微博热搜数据
  pub async fn weibo_hot_search_r(weibo_title: Option<String>, occur_era: Option<String>)
                                  -> Result<Vec<Self>, WeiboError> {
    let weibo_db_rb_conn = RBatis::new();
    weibo_db_rb_conn.link(SqliteDriver {}, WEIBO_DB_PTH).await?;

    let mut weibo_hot_search_r_qry = rbs::value! {};
    if let Some(weibo_title) = weibo_title {
      weibo_hot_search_r_qry.insert(rbs::value!("title"), rbs::value!(weibo_title));
    }
    if let Some(occur_era) = occur_era {
      weibo_hot_search_r_qry.insert(rbs::value!("occur_era"), rbs::value!(occur_era));
    }

    Self::select_by_map(&weibo_db_rb_conn, weibo_hot_search_r_qry).await.map_err(|flaw| {
      WeiboError::NjordError(flaw.to_string())
    })
  }

  /// 更新微博热搜WeiboHotSearch数据，如果有当天的同名的热搜，那么不做处理；否则直接插入。
  ///
  /// ## 参数
  /// - `hot_search_arrs`: 新的微博热搜数据
  pub async fn weibo_hot_search_u(hot_search_arrs: Vec<WeiboHotSearch>) -> Result<(), WeiboError> {
    if hot_search_arrs.is_empty() {
      return Ok(());
    }

    let weibo_db_rb_conn = RBatis::new();
    weibo_db_rb_conn.link(SqliteDriver {}, WEIBO_DB_PTH).await?;

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
      WeiboError::NjordError(flaw.to_string())
    })
  }

  /// 删除微博热搜WeiboHotSearch数据。
  ///
  /// ## 参数
  /// - `no_sieve`: 无筛选条件，删除全部数据时的保证参数
  /// - `occur_era`: 热搜出现的年月日，格式YYYY-MM-DD，可选
  pub async fn weibo_hot_search_d(no_sieve: bool, occur_era: Option<String>)
                                  -> Result<(), WeiboError> {
    let weibo_db_rb_conn = RBatis::new();
    weibo_db_rb_conn.link(SqliteDriver {}, WEIBO_DB_PTH).await?;

    let mut weibo_hot_search_d_qry = rbs::value! {};

    if let Some(occur_era) = occur_era {
      weibo_hot_search_d_qry.insert(rbs::value!("occur_era"), rbs::value!(occur_era));
    } else if !no_sieve {
      return Err(WeiboError::NjordError("delete all the data from the database, \
                                         but no_sieve guarantee isn't provided.".to_string()));
    }

    Self::delete_by_map(&weibo_db_rb_conn, weibo_hot_search_d_qry).await.map(|_| ()).map_err(
      |flaw| {
        WeiboError::NjordError(flaw.to_string())
      }
    )
  }
}


// /// 微博热门推荐
// #[derive(Debug, Table)]
// #[table_name = "weibo_hot_timeline"]
// pub struct WeiboHotTimeline {
//   pub id: AutoIncrementPrimaryKey<usize>,
//   pub mid: String,
//   pub text: String,
//   pub user_id: String,
//   pub user_name: String,
// }
//
//
// /// 微博评论
// #[derive(Debug, Table)]
// #[table_name = "weibo_comment"]
// pub struct WeiboComment {
//   pub id: AutoIncrementPrimaryKey<usize>,
//   pub mid: String,
//   pub text: String,
//   pub user_id: String,
//   pub user_name: String,
//   pub comment_era: String,
//   // 是否是评论回复
//   pub reply: bool,
//   // 如果是评论回复，存储其根评论的id
//   pub comment_senior_id: String,
// }
//
//
// // TODO: 替换Njord
