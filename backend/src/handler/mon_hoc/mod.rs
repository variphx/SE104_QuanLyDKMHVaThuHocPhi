use axum::{extract::State, Json, Router};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Serialize, sqlx::FromRow)]
struct MonHoc {
    id: String,
    id_khoa: String,
    ten: String,
    loai: String,
    so_tiet: i64,
}

#[derive(Deserialize)]
struct MonHocCreatePayload {
    id: String,
    id_khoa: String,
    ten: String,
    loai: String,
    so_tiet: String,
}

#[derive(Deserialize)]
struct MonHocQueryPayload {
    id: String,
}

#[derive(Deserialize)]
struct MonHocModifyPayload {
    id: String,
    payload: MonHocCreatePayload,
}

pub fn router() -> Router<Context> {
    Router::new()
        .route("/get", axum::routing::post(get))
        .route("/post", axum::routing::post(post))
        .route("/patch", axum::routing::post(patch))
        .route("/delete", axum::routing::post(delete))
}

async fn get(
    State(context): State<Context>,
    Json(payload): Json<MonHocQueryPayload>,
) -> Result<Json<MonHoc>, ()> {
    let mon_hoc = sqlx::query_as::<_, MonHoc>(
        "SELECT * FROM MON_HOC
            WHERE id = $1",
    )
    .bind(payload.id)
    .fetch_one(context.pool())
    .await
    .unwrap();

    Ok(Json(mon_hoc))
}

async fn post(State(context): State<Context>, Json(payload): Json<MonHocCreatePayload>) {
    let so_tiet = payload.so_tiet.parse::<i64>().unwrap();
    sqlx::query(
        "INSERT INTO MON_HOC (id, id_khoa, ten, loai, so_tiet)
            VALUES (
                $1,
                $2,
                $3,
                $4,
                $5
            )",
    )
    .bind(payload.id)
    .bind(payload.id_khoa)
    .bind(payload.ten)
    .bind(payload.loai)
    .bind(so_tiet)
    .execute(context.pool())
    .await
    .unwrap();
}

async fn patch(
    State(context): State<Context>,
    Json(MonHocModifyPayload { id, payload }): Json<MonHocModifyPayload>,
) {
    sqlx::query(
        "UPDATE MON_HOC
            SET ten = $1,
                so_tiet = $2
            WHERE id = $3",
    )
    .bind(payload.ten)
    .bind(payload.so_tiet)
    .bind(id)
    .execute(context.pool())
    .await
    .unwrap();
}

async fn delete(State(context): State<Context>, Json(payload): Json<MonHocQueryPayload>) {
    sqlx::query(
        "DELETE FROM MON_HOC
            WHERE id = $1",
    )
    .bind(payload.id)
    .execute(context.pool())
    .await
    .unwrap();
}
