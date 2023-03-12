// Typically, we should be able to complete the request within one service. 
// So we might combine route layer and service layer into one layer


use axum::{Json, extract::Path};
use crate::services::*;
use crate::models::promotion_model::*;

pub async fn get_promotion(Path(promotion_id):Path<String>) -> Json<Promotion> {
  dbg!(&promotion_id);
  let promotion = promotion_service::get_promotion(promotion_id).await;
  Json(promotion)
}

pub async fn get_all_promotions() -> Json<Vec<Promotion>> {
  let promotions = promotion_service::get_all_promotions().await;
  Json(promotions)
}