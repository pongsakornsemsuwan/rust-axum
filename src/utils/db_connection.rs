use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row, Pool, Postgres};

pub async fn get_connection_pool() -> Pool<Postgres> {
  let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost/postgres")
        .await;

  pool
}