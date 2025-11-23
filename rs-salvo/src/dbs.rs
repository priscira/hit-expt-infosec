use njord::column::Column;
use njord::condition::Condition;
use njord::condition::Value;
use njord::keys::AutoIncrementPrimaryKey;
use njord::sqlite;
use njord::table::Table;
use njord_derive::Table;
use crate::exceptions::WeiboError;
use crate::prefs::WEIBO_DB_PTH;

/// 避免SQL注入：将所有单引号'替换为两个单引号''，并用单引号包围整个字符串。
fn furnish_safe_sql(sql_talks: &str) -> String {
  format!("'{}'", sql_talks.replace('\'', "''"))
}

/// 微博热搜
#[derive(Debug, Table)]
#[table_name = "weibo_hot_search"]
pub struct WeiboHotSearch {
  pub id: AutoIncrementPrimaryKey<usize>,
  // 热搜标题
  pub title: String,
  // 热搜热度
  pub number: u32,
  // 热搜是否为特殊热搜，比如“热”、“新”
  pub special: String,
  // 热搜出现的时间，格式为YYYY-MM-DD
  pub occur_era: String,
}

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
      id: AutoIncrementPrimaryKey::default(),
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
  pub fn weibo_hot_search_r(weibo_title: Option<String>, occur_era: Option<String>)
                            -> Result<Vec<Self>, WeiboError> {
    let weibo_db_pth = std::path::Path::new(WEIBO_DB_PTH);
    let weibo_db_conn = sqlite::open(weibo_db_pth).map_err(|flaw| {
      WeiboError::NjordError(flaw.to_string())
    })?;

    let mut weibo_hot_search_r_qry = sqlite::select(vec![
      Column::Text("id".to_string()), Column::Text("title".to_string()),
      Column::Text("number".to_string()), Column::Text("special".to_string()),
      Column::Text("occur_era".to_string()),
    ]).from(WeiboHotSearch::default());

    if let Some(weibo_title) = weibo_title {
      weibo_hot_search_r_qry = weibo_hot_search_r_qry.where_clause(
        Condition::Eq("title".to_string(), Value::Literal(weibo_title))
      );
    }
    if let Some(occur_era) = occur_era {
      weibo_hot_search_r_qry = weibo_hot_search_r_qry.where_clause(
        Condition::Eq("occur_era".to_string(), Value::Literal(occur_era))
      )
    }

    weibo_hot_search_r_qry.build(&weibo_db_conn).map_err(|flaw| {
      WeiboError::NjordError(flaw.to_string())
    })
  }

  /// 更新微博热搜WeiboHotSearch数据，如果有当天的同名的热搜，那么不做处理；否则直接插入。
  ///
  /// ## 参数
  /// - `hot_search_arrs`: 新的微博热搜数据
  pub fn weibo_hot_search_u(hot_search_arrs: Vec<WeiboHotSearch>) -> Result<(), WeiboError> {
    if hot_search_arrs.is_empty() {
      return Ok(());
    }

    let weibo_db_pth = std::path::Path::new(WEIBO_DB_PTH);
    let weibo_db_conn = sqlite::open(weibo_db_pth).map_err(|flaw| {
      WeiboError::NjordError(flaw.to_string())
    })?;

    // Njord暂不支持DoNothing配置，选择执行原生SQL
    // sqlite::insert(&weibo_db_conn, hot_search_arrs).map_err(|flaw| {
    //   WeiboError::NjordError(flaw.to_string())
    // })?;

    let weibo_hot_search_pars = hot_search_arrs.iter().map(|hot_search_arri| {
      format!("({}, {}, {}, {})",
              furnish_safe_sql(&hot_search_arri.title),
              hot_search_arri.number,
              furnish_safe_sql(&hot_search_arri.special),
              furnish_safe_sql(&hot_search_arri.occur_era))
    }).collect::<Vec<String>>().join(", ");
    let weibo_hot_search_u_qry = format!(
      "insert or ignore into {} (title, number, special, occur_era) values {}",
      WeiboHotSearch::default().get_name(),
      weibo_hot_search_pars
    );

    sqlite::raw_execute(&weibo_db_conn, &weibo_hot_search_u_qry)?;
    Ok(())
  }

  /// 删除微博热搜WeiboHotSearch数据。
  ///
  /// ## 参数
  /// - `no_sieve`: 无筛选条件，删除全部数据时的保证参数
  /// - `occur_era`: 热搜出现的年月日，格式YYYY-MM-DD，可选
  pub fn weibo_hot_search_d(no_sieve: bool, occur_era: Option<String>) -> Result<(), WeiboError> {
    let weibo_db_pth = std::path::Path::new(WEIBO_DB_PTH);
    let weibo_db_conn = sqlite::open(weibo_db_pth).map_err(|flaw| {
      WeiboError::NjordError(flaw.to_string())
    })?;

    let mut weibo_hot_search_d_qry = sqlite::delete().from(WeiboHotSearch::default());

    if let Some(occur_era) = occur_era {
      weibo_hot_search_d_qry = weibo_hot_search_d_qry.where_clause(
        Condition::Eq("occur_era".to_string(), Value::Literal(occur_era))
      )
    } else if !no_sieve {
      return Err(WeiboError::NjordError("delete all the data from the database, \
                                         but no_sieve guarantee isn't provided.".to_string()));
    }

    weibo_hot_search_d_qry.build(&weibo_db_conn).map_err(|flaw| {
      WeiboError::NjordError(flaw.to_string())
    })
  }
}


/// 微博热门推荐
#[derive(Debug, Table)]
#[table_name = "weibo_hot_timeline"]
pub struct WeiboHotTimeline {
  pub id: AutoIncrementPrimaryKey<usize>,
  pub mid: String,
  pub text: String,
  pub user_id: String,
  pub user_name: String,
}


/// 微博评论
#[derive(Debug, Table)]
#[table_name = "weibo_comment"]
pub struct WeiboComment {
  pub id: AutoIncrementPrimaryKey<usize>,
  pub mid: String,
  pub text: String,
  pub user_id: String,
  pub user_name: String,
  pub comment_era: String,
  // 是否是评论回复
  pub reply: bool,
  // 如果是评论回复，存储其根评论的id
  pub comment_senior_id: String,
}


// TODO: 替换Njord
