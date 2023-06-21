use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, Serialize, ToSchema)]
pub enum ApiError {
    InternalServerError,
}

impl std::error::Error for ApiError {}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::InternalServerError => f.write_str("an internal server error occured"),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    error: ApiError,
    description: String,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let error = ErrorResponse {
            error: *self,
            description: self.to_string(),
        };
        HttpResponse::build(self.status_code()).json(&error)
    }
}
