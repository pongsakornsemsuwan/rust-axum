
use std::str::FromStr;

use log::LevelFilter;
use sqlx::postgres::{PgPoolOptions, PgConnectOptions};
use sqlx::{Pool, Postgres, Error, ConnectOptions};

pub async fn get_connection_pool() -> Result<Pool<Postgres>, Error> {
  // We have 4 options to handle things here
  // 1. using match and match both Ok and Err arms (Normal way)
  // 2. using await.unwrap() to panic! on Err
  // 3. using await.expect() to panic! with message (still panic tho)
  // 4. using await? to return Ok(_) and Err(err)  -- incase we don't want to transform error 
  let options = PgConnectOptions::from_str("postgres://postgres:postgres@localhost/postgres")
    .unwrap().log_statements(LevelFilter::Debug).clone();

  let pool =  PgPoolOptions::new()
    .max_connections(5) //TODO: read max connection from env instead
    .connect_with(options)
    .await
    .expect("Should be able to obtain Pool");
    Ok(pool)
}