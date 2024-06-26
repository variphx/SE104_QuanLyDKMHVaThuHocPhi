use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router};
use serde::Serialize;

use crate::context::Context;

pub fn router() -> Router<Context> {
    Router::new().route("/get", axum::routing::get(get))
}

#[derive(Serialize, sqlx::FromRow)]
struct Nganh {
    id: String,
    id_khoa: String,
    ten: String,
}

async fn get(State(context): State<Context>) -> impl IntoResponse {
    match sqlx::query_as::<_, Nganh>("select id, id_khoa, ten from nganh")
        .fetch_all(context.pool())
        .await
    {
        Ok(x) => Json(x).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
