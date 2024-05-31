use axum::{
    routing::{delete, get, patch, post},
    Router,
};

mod ticket;

#[derive(Clone)]
pub struct AppState {
    // TODO
}

pub fn router(state: AppState) -> Router {
    Router::new()
        // Root(/) route
        .route("/", get(|| async { "Hello, Aksem!" }))
        // Ticket(/tickets/**) routes
        .route("/tickets", post(ticket::create))
        .route("/tickets/:id", delete(ticket::delete))
        .route("/tickets", patch(ticket::update))
        .route("/tickets", get(ticket::list))
        .route("/tickets/:id", get(ticket::get))
        .with_state(state)
}
