use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct Promotion {
  pub name: String,
  // pub created_at: sqlx::types::chrono::DateTime<chrono::Utc>,
}