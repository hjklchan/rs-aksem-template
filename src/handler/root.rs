use axum::response::IntoResponse;

use crate::result::OhMyResult;

pub async fn root_handler() -> impl IntoResponse {
    let ok: OhMyResult<&str> = Ok("It's ok");
    ok
}
