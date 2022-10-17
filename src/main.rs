#[macro_use]
extern crate diesel;

use diesel::RunQueryDsl;
use diesel_async::pg::AsyncPgConnection;
use diesel_async::pooled_connection::{bb8::Pool, AsyncDieselConnectionManager};

table! {
  person (id) {
    id -> Int4,
    name -> Text,
  }
}

pub type DbPool = Pool<AsyncPgConnection>;

#[tokio::main]
async fn main() {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new("test");
    let pool = Pool::builder().build(config).await.unwrap();
    let mut conn = pool.get().await.unwrap();
    let output = diesel::delete(person::table).execute(&mut conn).unwrap();
}
