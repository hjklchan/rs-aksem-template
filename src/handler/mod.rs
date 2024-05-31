pub mod root;
pub mod tickets;

use axum::{
    extract::State,
    routing::{delete, get, patch, post},
    Router,
};
use root::root_handler;

/// 路由
pub fn routes<S>(state: State<S>) -> Router
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(root_handler))
        // Ticket 模块
        .route("/tickets", post(tickets::create_handler))
        .route("/tickets/:id", delete(tickets::delete_handler))
        .route("/tickets/:id", patch(tickets::update_handler))
        .route("/tickets/:id", get(tickets::get_handler))
        .route("/tickets", get(tickets::list_handler))
        .with_state(state)
}
