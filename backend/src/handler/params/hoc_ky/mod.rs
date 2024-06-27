use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

use crate::context::Context;

#[derive(Serialize, sqlx::FromRow)]
struct HocKy {
    id: String,
    nam_hoc: i32,
    ten: String,
}

pub async fn get(State(context): State<Context>) -> impl IntoResponse {
    match sqlx::query_as::<_, HocKy>(
        "select hoc_ky.id, hoc_ky.nam_hoc, hoc_ky.ten from tham_so, hoc_ky
            where tham_so.id_hoc_ky = hoc_ky.id
                and tham_so.id = 1",
    )
    .fetch_one(context.pool())
    .await
    {
        Ok(x) => Json(x).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
