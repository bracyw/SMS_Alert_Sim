use axum::response::{IntoResponse, Response};

/// Custom error type for the historical handler (AI generated)
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    DbErr(#[from] sea_orm::DbErr),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
