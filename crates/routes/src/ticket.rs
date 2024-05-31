use axum::response::IntoResponse;

pub async fn create() -> impl IntoResponse {
    "Create a new ticket"
}

pub async fn delete() -> impl IntoResponse {
    "Delete ticket"
}

pub async fn update() -> impl IntoResponse {
    "Update ticket"
}

pub async fn get() -> impl IntoResponse {
    "Get ticket"
}

pub async fn list() -> impl IntoResponse {
    "Get tickets"
}
