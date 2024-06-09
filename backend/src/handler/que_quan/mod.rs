use axum::{extract::State, Json, Router};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Deserialize)]
struct QueQuanQueryPayload {
    id: String,
}

#[derive(Serialize)]
struct QueQuan {}

pub fn router() -> Router<Context> {
    Router::new().route("/get", axum::routing::post(get))
}

async fn get(
    State(state): State<Context>,
    Json(payload): Json<QueQuanQueryPayload>,
) -> Result<Json<QueQuan>, Json<String>> {
    todo!()
}
