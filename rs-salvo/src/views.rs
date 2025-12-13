use nyquest::AsyncClient;
use rbatis::RBatis;
use salvo::prelude::*;

#[handler]
pub async fn hello(depot: &mut Depot) -> String {
  let custom_data: &AsyncClient = depot.get("weibo_clt").unwrap();
  println!("####{:?}", custom_data);
  let weibo_db_rb_conn: &RBatis = depot.get("weibo_db_rb_conn").unwrap();
  println!("####{:?}", weibo_db_rb_conn);
  "Hello World".to_string()
}
