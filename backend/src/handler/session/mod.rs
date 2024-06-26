use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router};
use serde::{Deserialize, Serialize};

use crate::context::Context;

pub fn router() -> Router<Context> {
    Router::new()
        .route("/get", axum::routing::post(get))
        .route("/post", axum::routing::post(post))
}

#[derive(Deserialize)]
struct SessionCreatePayload {
    id: String,
    username: String,
}

#[derive(Serialize, sqlx::FromRow)]
struct Session {
    id: String,
    username: String,
}

async fn post(
    State(context): State<Context>,
    Json(SessionCreatePayload {
        username,
        id: session_id,
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

async fn get(State(context): State<Context>, Json(id): Json<String>) -> impl IntoResponse {
    match sqlx::query_as::<_, Session>(
        "select id, username from session
            where id = $1",
    )
    .bind(id)
    .fetch_one(context.pool())
    .await
    {
        Ok(x) => Json(x).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}
