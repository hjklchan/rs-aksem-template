pub mod db;
mod status;

pub use status::Status;
use axum::{response::IntoResponse, Json};
use thiserror::Error;

/// ### 此处定义错误集
#[derive(Debug, Error)]
pub enum ApiError {
    // 数据库相关的错误
    #[error(transparent)]
    Database(#[from] db::DatabaseError),
    // TODO: 模块级的错误示例
}

/// ### 为错误集实现 IntoResponse 特征
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status: Status<()> = match self {
            Self::Database(err) => Status::Bad(10001, err.to_string()),
        };
        // Send response
        Json(status.to_reply()).into_response()
    }
}
