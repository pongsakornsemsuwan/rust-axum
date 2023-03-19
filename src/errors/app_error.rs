use crate::models::promotion_model::Promotion;

use super::promotion_service_error::PromotionServiceError;
use axum::{self, response::{Response, IntoResponse}, http::StatusCode, Json};
use serde_json::json;
pub enum AppError {
    /// Something went wrong when calling the user repo.
    DefaultError,
    PromotionService(PromotionServiceError),
}


impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            // DEFAULT ERROR
            AppError::DefaultError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),

            // Promotion Service Error
            AppError::PromotionService(PromotionServiceError::NotFound) => (StatusCode::IM_A_TEAPOT, "default error"),
            AppError::PromotionService(PromotionServiceError::SomeOtherError)=> (StatusCode::INTERNAL_SERVER_ERROR, "Some other error"),
            AppError::PromotionService(PromotionServiceError::YetAnotherError)=> (StatusCode::INTERNAL_SERVER_ERROR, "Default error"),

            
        };
        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
