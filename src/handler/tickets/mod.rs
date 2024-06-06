mod create;
mod delete;
mod get;
mod list;
mod update;

use ::chrono::Utc;
pub use create::create_handler;
pub use delete::delete_handler;
pub use get::get_handler;
pub use list::list_handler;
use sqlx::types::chrono::{self, Local};
pub use update::update_handler;

#[derive(Debug, serde::Serialize)]
pub struct Ticket {
    pub id: u64,
    pub assignee_id: Option<u64>, // ! Should related to User(Doesn't design it now)
    pub title: String,
    pub description: Option<String>,
    pub body: Option<String>,
    pub status: i8,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
}
