use axum::{extract::State, Json};

use crate::context::Context;

pub async fn get(State(context): State<Context>) -> Result<Json<String>, ()> {
    let id_hoc_ky_hien_tai = sqlx::query_scalar::<_, String>(
        "SELECT id_hoc_ky_hien_tai FROM THAM_SO
            WHERE id = 1",
    )
    .fetch_one(context.pool())
    .await
    .unwrap();

    Ok(Json(id_hoc_ky_hien_tai))
}
