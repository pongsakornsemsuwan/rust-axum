use axum::response::IntoResponse;
use sqlx::{Pool, Error};
use sqlx::{Postgres};
use uuid::Uuid;

use crate::errors::app_error::AppError;
use crate::errors::promotion_service_error::PromotionServiceError::{*, self};
use crate::models::promotion_model::Promotion;
use crate::dao::promotion_dao;

// Return Option because we might not find
pub async fn get_promotion(pool: Pool<Postgres>, id: Uuid) -> Result<Promotion, PromotionServiceError> {
  let conn = pool.acquire().await.unwrap();
  // conn.begin();
  match promotion_dao::get_promotion_by_id(conn, id).await {
    Some(promotion) => Ok(promotion),
    _ => Err(PromotionServiceError::NotFound),
  }
}

pub async fn get_all_promotions(pool: Pool<Postgres>) -> Result<Vec<Promotion>, AppError> {
  let conn = match pool.acquire().await {
    Ok(conn) => conn,
    _ => { return Err(AppError::DefaultError); }
  };
  let promotions = promotion_dao::get_all_promotions(conn).await?;
  Ok(promotions)
}  

pub async fn update_promotion(pool: Pool<Postgres>, promotion_id:Uuid, name: String) -> Result<Promotion, AppError> {
  let conn = pool.acquire().await.unwrap();
  let promotion = promotion_dao::update_promotion(conn, promotion_id, name).await?;
  Ok(promotion)
}

pub async fn add_promotion(pool: Pool<Postgres>, name: String) -> Result<Promotion, impl IntoResponse> {
  let conn = pool.acquire().await.unwrap();
  match promotion_dao::add_promotion(conn, name).await {
    Ok(promotion) => Ok(promotion),
    _ => Err(AppError::PromotionService(NotFound)),
  }
}