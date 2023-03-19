use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};

use super::app_error::AppError;

pub enum PromotionServiceError {
    NotFound,
    SomeOtherError,
    YetAnotherError,
}

/// This makes it possible to use `?` to automatically convert a `PromotionServiceError`
/// into an `AppError`.
impl From<PromotionServiceError> for AppError {
    fn from(inner: PromotionServiceError) -> Self {
        AppError::PromotionService(inner)
    }
}

