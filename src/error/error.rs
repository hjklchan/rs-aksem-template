use axum::response::IntoResponse;
use super::db::DatabaseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    Database(#[from] DatabaseError),

    // TODO: Make a example error
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Database(err) => err.into_response()
        }
    }
}
