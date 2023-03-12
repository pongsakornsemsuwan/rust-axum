use std::time::SystemTime;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Promotion {
  pub name: String,
  pub created_at: SystemTime,
}