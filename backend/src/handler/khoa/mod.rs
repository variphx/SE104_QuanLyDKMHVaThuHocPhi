use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::context::Context;

mod delete;
mod get;
mod patch;
mod post;

#[derive(Serialize, sqlx::FromRow)]
struct Khoa {
    id: String,
    ten: String,
}

#[derive(Deserialize)]
struct KhoaCreatePayload {
    id: String,
    ten: String,
}

#[derive(Deserialize)]
struct KhoaQueryPayload {
    id: String,
}

#[derive(Deserialize)]
struct KhoaModifyPayload {
    id: String,
    ten: String,
}

pub fn method_router() -> axum::routing::MethodRouter<Context> {
    axum::routing::get(get)
        .post(post)
        .patch(patch)
        .delete(delete)
}

async fn get(
    State(context): State<Context>,
    Json(KhoaQueryPayload { id }): Json<KhoaQueryPayload>,
) -> Result<Json<Khoa>, ()> {
    let khoa = sqlx::query_as::<_, Khoa>(
        "SELECT * FROM KHOA
            WHERE id = $1",
    )
    .bind(id)
    .fetch_one(context.pool())
    .await
    .unwrap();

    Ok(Json(khoa))
}

async fn post(
    State(context): State<Context>,
    Json(payload): Json<KhoaCreatePayload>,
) -> Result<(), ()> {
    sqlx::query(
        "INSERT INTO KHOA
            VALUES (
                $1,
                $2
            )",
    )
    .bind(payload.id)
    .bind(payload.ten)
    .execute(context.pool())
    .await
    .unwrap();

    Ok(())
}

async fn patch(State(context): State<Context>, Json(payload): Json<KhoaModifyPayload>) {
    sqlx::query(
        "UPDATE KHOA
            SET ten = $1
            WHERE id = $2",
    )
    .bind(payload.ten)
    .bind(payload.id)
    .execute(context.pool())
    .await
    .unwrap();
}

async fn delete(State(context): State<Context>, Json(payload): Json<KhoaQueryPayload>) {
    sqlx::query(
        "DELETE FROM KHOA
            WHERE id = $1",
    )
    .bind(payload.id)
    .execute(context.pool())
    .await
    .unwrap();
}
