use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Serialize, sqlx::FromRow)]
struct ThanhPho {
    id: String,
    ten: String,
    id_tinh: String,
}

#[derive(Deserialize)]
struct QueQuanQueryPayload {
    id_thanh_pho: String,
}

pub fn method_router() -> axum::routing::MethodRouter<Context> {
    axum::routing::get(get)
}

async fn get(
    State(context): State<Context>,
    Json(payload): Json<QueQuanQueryPayload>,
) -> Json<ThanhPho> {
    let que_quan = sqlx::query_as::<_, ThanhPho>(
        "SELECT * FROM QUE_QUAN
            WHERE id = $1",
    )
    .bind(payload.id_thanh_pho)
    .fetch_one(context.pool())
    .await
    .unwrap();

    Json(que_quan)
}
