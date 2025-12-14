use nyquest::AsyncClient;
use rbatis::RBatis;
use salvo::prelude::*;

#[handler]
pub async fn hello(depot: &mut Depot) -> String {
  let custom_data: &AsyncClient = depot.get("weibo_clt").unwrap();
  let weibo_db_rb_conn: &RBatis = depot.get("weibo_db_rb_conn").unwrap();
  println!("####");
  "Hello World".to_string()
}
