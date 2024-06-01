/// Axum 全局状态数据结构
#[derive(Clone)]
pub struct AppState {
    pub db: (),
}

pub fn new(db: ()) -> AppState {
    AppState { db }
}