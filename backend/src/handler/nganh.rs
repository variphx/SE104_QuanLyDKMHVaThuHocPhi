use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Serialize, sqlx::FromRow)]
struct Nganh {
    id: String,
    id_khoa: String,
    ten: String,
}

#[derive(Deserialize)]
struct NganhQueryPayload {
    id: String,
}

#[derive(Deserialize)]
struct NganhCreatePayload {
    id: String,
    ten: String,
    id_khoa: String,
}

#[derive(Deserialize)]
struct NganhModifyPayload {
    id: String,
    ten: String,
}

pub fn method_router() -> axum::routing::MethodRouter<Context> {
    axum::routing::get(get)
}

async fn get(
    State(context): State<Context>,
    Json(payload): Json<NganhQueryPayload>,
) -> Result<Json<Nganh>, ()> {
    let nganh = sqlx::query_as::<_, Nganh>(
        "SELECT * FROM NGANH
            WHERE id = $1",
    )
    .bind(payload.id)
    .fetch_one(context.pool())
    .await
    .unwrap();

    Ok(Json(nganh))
}
