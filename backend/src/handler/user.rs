use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{extract::State, http::StatusCode, Json};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Deserialize)]
struct User {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResult {
    is_success: bool,
}

pub fn method_router() -> axum::routing::MethodRouter<Context> {
    axum::routing::get(get)
        .post(post)
        .patch(patch)
        .delete(delete)
}

async fn get(
    State(context): State<Context>,
    Json(User { username, password }): Json<User>,
) -> Json<LoginResult> {
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
        Ok(()) => Json(LoginResult { is_success: true }),
        Err(_) => Json(LoginResult { is_success: false }),
    }
}

async fn post(State(context): State<Context>, Json(user): Json<User>) -> Result<(), StatusCode> {
    let User { username, password } = user;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    sqlx::query(
        "INSERT INTO USERS (username, password)
            VALUES (
                $1,
                $2
            )",
    )
    .bind(username)
    .bind(password_hash)
    .execute(context.pool())
    .await
    .unwrap();

    Ok(())
}

async fn patch(State(context): State<Context>, Json(user): Json<User>) -> Result<(), StatusCode> {
    let User { username, password } = user;

    let username = sqlx::query_scalar::<_, String>(
        "SELECT username FROM USERS
                WHERE username = $1",
    )
    .bind(&username)
    .fetch_one(context.pool())
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    sqlx::query(
        "UPDATE USERS
            SET password = $1
            WHERE username = $2",
    )
    .bind(password_hash)
    .bind(username)
    .execute(context.pool())
    .await
    .unwrap();

    Ok(())
}

async fn delete(
    State(context): State<Context>,
    Json(username): Json<String>,
) -> Result<(), StatusCode> {
    sqlx::query(
        "DELETE FROM USERS
            WHERE username = $1",
    )
    .bind(username)
    .execute(context.pool())
    .await
    .unwrap();

    Ok(())
}