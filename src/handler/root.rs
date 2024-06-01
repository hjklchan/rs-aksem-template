use axum::response::IntoResponse;

use crate::result::{error::ApiError, OhMyResult};

/// Test ok
pub async fn root_handler() -> impl IntoResponse {
    let is_ok: OhMyResult<()> = Err(ApiError::Database(sqlx::Error::ColumnNotFound(
        "xmy".into(),
    )));

    is_ok
}
