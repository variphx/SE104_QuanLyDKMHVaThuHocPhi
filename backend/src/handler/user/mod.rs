use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Deserialize, Serialize, sqlx::FromRow)]
struct User {
    username: String,
    hash_pwd: String,
}

#[derive(Deserialize)]
struct UserLoginPayload(String);

pub fn router() -> Router<Context> {
    Router::new()
        .route("/get", axum::routing::post(get))
        .route("/post", axum::routing::post(post))
        .route("/patch", axum::routing::post(patch))
        .route("/delete", axum::routing::post(delete))
}

async fn get(
    State(context): State<Context>,
    Json(UserLoginPayload(username)): Json<UserLoginPayload>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, User>(
        "select username, password as hash_pwd from users
            where username = $1",
    )
    .bind(username.as_str())
    .fetch_one(context.pool())
    .await
    {
        Ok(x) => Json(x).into_response(),
        Err(error) => (StatusCode::NOT_FOUND, error.to_string()).into_response(),
    }
}

async fn post(State(context): State<Context>, Json(user): Json<User>) -> Result<(), StatusCode> {
    let User {
        username,
        hash_pwd: password,
    } = user;

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
    let User {
        username,
        hash_pwd: password,
    } = user;

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
