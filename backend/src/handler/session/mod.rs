use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router};

use crate::context::Context;

pub fn router() -> Router<Context> {
    Router::new()
        .route("/get", axum::routing::post(get))
        .route("/post", axum::routing::post(post))
}

#[derive(serde::Deserialize)]
struct SessionCreatePayload {
    session_id: i64,
    username: String,
}

async fn post(
    State(context): State<Context>,
    Json(SessionCreatePayload {
        username,
        session_id,
    }): Json<SessionCreatePayload>,
) -> impl IntoResponse {
    match sqlx::query(
        "insert into session (id, username)
            values (
                $1,
                $2
            )",
    )
    .bind(session_id)
    .bind(username)
    .execute(context.pool())
    .await
    {
        Ok(_) => (StatusCode::CREATED).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}

async fn get(State(context): State<Context>, Json(session_id): Json<i64>) -> impl IntoResponse {
    match sqlx::query_scalar::<_, String>(
        "select username from session
            where id = $1",
    )
    .bind(session_id)
    .fetch_one(context.pool())
    .await
    {
        Ok(value) => Json(value).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}
