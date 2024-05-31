use axum::{response::IntoResponse, Json};
use crate::result::status::Status;

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
        let status: Status<()> = match self {
            Self::Database(err) => Status::Bad(10001, err.to_string()),
        };
        // Send response
        Json(status.to_reply()).into_response()
    }
}
