use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router};
use serde::Serialize;

use crate::context::Context;

pub fn router() -> Router<Context> {
    Router::new().route("/get", axum::routing::post(get))
}

#[derive(Serialize, sqlx::FromRow)]
struct ThanhPho {
    id: String,
    ten: String,
}

async fn get(State(context): State<Context>, Json(id_tinh): Json<String>) -> impl IntoResponse {
    match sqlx::query_as::<_, ThanhPho>(
        "select id, ten from thanh_pho
            where thanh_pho.id_tinh = $1",
    )
    .bind(id_tinh)
    .fetch_all(context.pool())
    .await
    {
        Ok(value) => Json(value).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}
