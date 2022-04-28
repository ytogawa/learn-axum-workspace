use axum::{http::StatusCode, response::IntoResponse, Json};
use common::error::Error;

pub enum AppError {
    Error(Error),
}

impl From<Error> for AppError {
    fn from(inner: Error) -> Self {
        AppError::Error(inner)
    }
}

// FIXME この辺要検討
type S = (StatusCode, &'static str);
static INTERNAL_SERVER_ERROR: S = (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error");
static NOT_FOUND: S = (StatusCode::NOT_FOUND, "Not Found");

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::Error(e) => match e {
                Error::NotFound(_, _) => NOT_FOUND,
                _ => INTERNAL_SERVER_ERROR,
            },
        };
        let body = Json(serde_json::json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
