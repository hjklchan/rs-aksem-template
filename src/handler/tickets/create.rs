use axum::extract::State;

use crate::{app_state::AppState, result::OhMyResult};

pub async fn create_handler(State(state): State<AppState>) -> OhMyResult<()> {
    OhMyResult::Ok(())
}
