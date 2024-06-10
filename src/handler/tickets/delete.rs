use axum::extract::{Path, State};

use crate::{
    app_state::AppState,
    result::{
        error::{db::DatabaseError, ticket::TicketError},
        OhMyResult,
    },
};

pub async fn delete_handler(
    State(state): State<AppState>,
    Path(ticket_id): Path<u64>,
) -> OhMyResult<()> {
    let sql = "DELETE FROM `tickets` WHERE `id` = ?";
    let rows_affected = sqlx::query(sql)
        .bind(ticket_id)
        .execute(&state.db)
        .await
        .map(|result| result.rows_affected())
        .map_err(|err| DatabaseError(err))?;

    if rows_affected == 0 {
        return Err(TicketError::NotFound.into());
    }

    OhMyResult::Ok(())
}
