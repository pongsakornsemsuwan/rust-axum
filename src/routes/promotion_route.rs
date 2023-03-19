use axum::extract::State;
use axum::http::StatusCode;
use axum::{Json, extract::Path};
use uuid::Uuid;
use crate::errors::app_error::AppError;
use crate::utils::str_to_uuid;
use crate::{services::*, AppState};
use crate::models::promotion_model::*;

// I don't know why can't I extract Uuid from path directly, have to extract String and convert
pub async fn get_promotion(State(state): State<AppState>, Path(promotion_id):Path<String>) -> Result<Json<Promotion>, AppError> {
  let promotion = promotion_service::get_promotion(state.pool, Uuid::parse_str(promotion_id.as_str()).unwrap()).await?;
  Ok(Json(promotion))
}


pub async fn get_all_promotions(State(state): State<AppState>) -> Result<Json<Vec<Promotion>>, AppError> {
  let promotions = promotion_service::get_all_promotions(state.pool).await?;
  Ok(Json(promotions))
}


pub async fn update_promotion(State(state): State<AppState>, Path(promotion_id):Path<String>, Json(body): Json<Promotion>) -> Result<Json<Promotion>, AppError> {
  let name = body.name;
  match promotion_service::update_promotion(
    state.pool,
    str_to_uuid(promotion_id), 
    name)
    .await {
      Ok(promotion) => Ok(Json(promotion)),
      Err(err) => Err(err)
    }
}


pub async fn add_promotion(State(state): State<AppState>, Json(body): Json<Promotion>) -> Result<Json<Promotion>, StatusCode> {
  let name = body.name;
  match promotion_service::add_promotion(state.pool, name).await {
    Ok(promotion) => Ok(Json(promotion)),
    _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
 }
}

