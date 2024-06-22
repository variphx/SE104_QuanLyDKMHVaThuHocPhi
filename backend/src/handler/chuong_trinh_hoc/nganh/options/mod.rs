use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router};
use serde::Serialize;

use crate::context::Context;

pub fn router() -> Router<Context> {
    Router::new().route("/get", axum::routing::post(get))
}

#[derive(Serialize, sqlx::FromRow)]
struct Nganh {
    id: String,
    ten: String,
}

async fn get(State(context): State<Context>, Json(id_khoa): Json<String>) -> impl IntoResponse {
    match sqlx::query_as::<_, Nganh>(
        "select id, ten from nganh
            where id_khoa = $1",
    )
    .bind(id_khoa)
    .fetch_all(context.pool())
    .await
    {
        Ok(value) => Json(value).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}
