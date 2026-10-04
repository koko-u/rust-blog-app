use axum::http;
use axum::response;

use crate::shared::responses;

#[derive(Debug, derive_more::Display, derive_more::Error)]
pub enum ApiError {
    #[display("Database error: {}", _0)]
    Database(#[error(source)] sqlx::Error),
    #[display("Validation error: {}", _0)]
    Validation(#[error(source)] garde::Report),
    #[display("Not found: {message}")]
    NotFound { message: String },
    #[display("Unauthorized error")]
    Unauthorized,
    #[display("Failed to create Slug: {message} (max = {max_count})")]
    Slug { message: String, max_count: u32 },
    #[display("Other error: {message}")]
    Other { message: String },
}

impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}
impl From<garde::Report> for ApiError {
    fn from(error: garde::Report) -> Self {
        Self::Validation(error)
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
pub enum ValidationError {
    #[display("Database error: {}", _0)]
    Database(#[error(source)] sqlx::Error),
    #[display("Validation error: {}", _0)]
    Validation(#[error(source)] garde::Report),
    #[display("Not found: {}", message)]
    NoResource { message: String },
}

impl response::IntoResponse for ApiError {
    fn into_response(self) -> response::Response {
        match &self {
            Self::Database(err) => {
                tracing::error!("Database error: {err:?}");
                let problem_details = responses::ProblemDetails::simple(
                    "Something went wrong",
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                );
                (
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(problem_details),
                )
                    .into_response()
            }
            Self::Validation(err) => {
                tracing::warn!("Validation error: {err:?}");
                let problem_details = responses::ProblemDetails::from(err);
                (problem_details.status, axum::Json(problem_details)).into_response()
            }
            Self::NotFound { message } => {
                tracing::info!("Not found: {message}");

                (http::StatusCode::NOT_FOUND, axum::Json(self.to_string())).into_response()
            }
            Self::Unauthorized => {
                tracing::warn!("Unauthorized error");
                let problem_details =
                    responses::ProblemDetails::simple("Unauthorized", http::StatusCode::UNAUTHORIZED);
                (http::StatusCode::UNAUTHORIZED, axum::Json(problem_details)).into_response()
            }
            Self::Slug { message, max_count } => {
                tracing::info!("Failed to create slug: {message} (max={max_count})");

                (
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(self.to_string()),
                )
                    .into_response()
            }
            Self::Other { message } => {
                tracing::info!("Other error: {message}");

                (
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(self.to_string()),
                )
                    .into_response()
            }
        }
    }
}

impl From<ValidationError> for ApiError {
    fn from(err: ValidationError) -> Self {
        match err {
            ValidationError::Database(err) => Self::Database(err),
            ValidationError::Validation(err) => Self::Validation(err),
            ValidationError::NoResource { message } => Self::NotFound { message },
        }
    }
}
