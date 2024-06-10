use axum::{extract::State, Json};

use crate::context::Context;

pub async fn get(State(context): State<Context>) -> Result<Json<String>, ()> {
    let current_nam_hoc = sqlx::query_scalar::<_, i32>(
        "SELECT nam_hoc_current FROM THAM_SO
            WHERE id = 1",
    )
    .fetch_one(context.pool())
    .await
    .unwrap();

    let current_id_hoc_ky = format!("{:04}01", current_nam_hoc);

    Ok(Json(current_id_hoc_ky))
}
