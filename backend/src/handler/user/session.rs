use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Deserialize, sqlx::FromRow)]
struct SessionCreatePayload {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct SessionCreateResponse {
    is_success: bool,
}

pub fn method_router() -> axum::routing::MethodRouter<Context> {
    axum::routing::post(post)
}

async fn post(
    State(context): State<Context>,
    Json(SessionCreatePayload { username, password }): Json<SessionCreatePayload>,
) -> Json<SessionCreateResponse> {
    let password_hash = sqlx::query_scalar::<_, String>(
        "SELECT password FROM USERS
            WHERE username = $1",
    )
    .bind(username)
    .fetch_one(context.pool())
    .await
    .unwrap();

    let parsed_hash = PasswordHash::new(&password_hash).unwrap();
    let argon2 = Argon2::default();

    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Json(SessionCreateResponse { is_success: true }),
        Err(_) => Json(SessionCreateResponse { is_success: false }),
    }
}
