/// Axum 全局状态数据结构
#[derive(Clone)]
pub struct AppState {
    db: (),
    other: (),
}

impl AppState {
    fn new() -> AppState {
        Self { db: (), other: () }
    }

    fn with_db(mut self, db: ()) -> Self {
        self.db = db;
        self
    }

    fn with_other(mut self, other: ()) -> Self {
        self.other = other;
        self
    }
}