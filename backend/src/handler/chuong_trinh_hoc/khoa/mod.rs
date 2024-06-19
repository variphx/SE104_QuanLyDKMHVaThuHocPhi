use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::context::Context;

#[derive(serde::Serialize, sqlx::FromRow)]
struct Khoa {
    ten: String,
}

pub async fn get(State(context): State<Context>, Json(id): Json<String>) -> impl IntoResponse {
    match sqlx::query_as::<_, Khoa>(
        "select ten from khoa
            where id = $1",
    )
    .bind(id)
    .fetch_one(context.pool())
    .await
    {
        Ok(value) => Json(value).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}
