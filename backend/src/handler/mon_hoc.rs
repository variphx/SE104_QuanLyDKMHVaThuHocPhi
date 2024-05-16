use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::context::Context;

pub mod mo;

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
    id_khoa: String,
    ten: String,
    loai: String,
    so_tiet: i64,
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

pub fn method_router() -> axum::routing::MethodRouter<Context> {
    axum::routing::get(get)
        .post(post)
        .patch(patch)
        .delete(delete)
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
    let id = {
        let mon_hoc_len = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT (*) FROM MON_HOC, KHOA
                WHERE MON_HOC.id_khoa = KHOA.id",
        )
        .fetch_one(context.pool())
        .await
        .unwrap();

        format!("{}{:04}", payload.id_khoa, mon_hoc_len + 1)
    };

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
    .bind(id)
    .bind(payload.id_khoa)
    .bind(payload.ten)
    .bind(payload.loai)
    .bind(payload.so_tiet)
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