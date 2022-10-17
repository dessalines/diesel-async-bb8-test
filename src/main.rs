#[macro_use]
extern crate diesel;

use diesel::RunQueryDsl;
use diesel_async::pooled_connection::{bb8::Pool, AsyncDieselConnectionManager};
use diesel_async::pg::AsyncPgConnection;

table! {
  person (id) {
    id -> Int4,
    name -> Text,
  }
}

pub type DbPool = Pool<AsyncPgConnection>;

#[tokio::main]
async fn main() {
    // let mut conn = pool.get().await.unwrap();
  let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new("test");
  let pool = Pool::builder().build(config).await.unwrap();
  let mut conn = pool.get().await.unwrap();
  let output = diesel::delete(person::table).execute(&mut conn).unwrap();
}
