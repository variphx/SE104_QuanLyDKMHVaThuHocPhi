use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router};
use serde::Serialize;

use crate::context::Context;

pub fn router() -> Router<Context> {
    Router::new().route("/get", axum::routing::get(get))
}

#[derive(Serialize, sqlx::FromRow)]
struct Khoa {
    id: String,
    ten: String,
}

async fn get(State(context): State<Context>) -> impl IntoResponse {
    match sqlx::query_as::<_, Khoa>("select id, ten from khoa")
        .fetch_all(context.pool())
        .await
    {
        Ok(value) => Json(value).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}
