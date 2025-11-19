use njord_derive::Table;
use njord::table::Table;
use njord::keys::AutoIncrementPrimaryKey;

#[derive(Debug, Table)]
#[table_name = "weibo_hot_search"]
pub struct WeiboHotSearch {
  pub id:         AutoIncrementPrimaryKey<usize>,
  pub title:      String,
  pub number:     u32,
  pub special:    String,
  pub occur_time: String
}
