use axum::{extract::State, http::StatusCode, Json, Router};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Serialize, sqlx::FromRow)]
struct SinhVien {
    id: String,
    ten: String,
    can_cuoc: String,
    ngay_sinh: String,
    so_dien_thoai: String,
    email: String,
    id_gioi_tinh: String,
    id_que_quan: String,
    id_doi_tuong: String,
    id_chuong_trinh_hoc: String,
}

#[derive(Deserialize)]
struct SinhVienCreatePayload {
    id: String,
    ten: String,
    ngay_sinh: String,
    id_gioi_tinh: String,
    id_que_quan: String,
    id_doi_tuong: String,
    id_chuong_trinh_hoc: String,
}

#[derive(Deserialize)]
struct SinhVienQueryPayload {
    id: String,
}

#[derive(Deserialize)]
struct SinhVienModifyPayload {
    id: String,
    payload: SinhVienCreatePayload,
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
    Json(payload): Json<SinhVienQueryPayload>,
) -> Json<SinhVien> {
    let sinh_vien = sqlx::query_as::<_, SinhVien>(
        "SELECT * FROM SINH_VIEN
            WHERE id = $1",
    )
    .bind(payload.id)
    .fetch_one(context.pool())
    .await
    .unwrap();

    Json(sinh_vien)
}

async fn post(
    State(context): State<Context>,
    Json(payload): Json<SinhVienCreatePayload>,
) -> Result<(), StatusCode> {
    let ngay_sinh = time::Date::parse(
        &payload.ngay_sinh,
        time::macros::format_description!("[year]-[month]-[day]"),
    )
    .unwrap();

    sqlx::query(
        "INSERT INTO SINH_VIEN (
            id,
            ten,
            ngay_sinh,
            id_gioi_tinh,
            id_que_quan,
            id_doi_tuong,
            id_chuong_trinh_hoc
        )
        VALUES (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7
        )",
    )
    .bind(&payload.id)
    .bind(&payload.ten)
    .bind(ngay_sinh)
    .bind(&payload.id_gioi_tinh)
    .bind(&payload.id_que_quan)
    .bind(&payload.id_doi_tuong)
    .bind(&payload.id_chuong_trinh_hoc)
    .execute(context.pool())
    .await
    .unwrap();

    Ok(())
}

async fn patch(
    State(context): State<Context>,
    Json(SinhVienModifyPayload { id, payload }): Json<SinhVienModifyPayload>,
) {
    sqlx::query(
        "DELETE FROM SINH_VIEN
            WHERE id = $1",
    )
    .bind(&id)
    .execute(context.pool())
    .await
    .unwrap();

    let ngay_sinh = time::Date::parse(
        &payload.ngay_sinh,
        time::macros::format_description!("[year]-[month]-[day]"),
    )
    .unwrap();

    sqlx::query(
        "INSERT INTO SINH_VIEN (
            id,
            ten,
            ngay_sinh,
            id_gioi_tinh,
            id_que_quan,
            id_doi_tuong,
            id_chuong_trinh_hoc
        )
        VALUES (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7
        )",
    )
    .bind(id)
    .bind(&payload.ten)
    .bind(ngay_sinh)
    .bind(&payload.id_gioi_tinh)
    .bind(&payload.id_que_quan)
    .bind(&payload.id_doi_tuong)
    .bind(&payload.id_chuong_trinh_hoc)
    .execute(context.pool())
    .await
    .unwrap();
}

async fn delete(State(context): State<Context>, Json(payload): Json<SinhVienQueryPayload>) {
    sqlx::query(
        "DELETE FROM SINH_VIEN
            WHERE id = $1",
    )
    .bind(payload.id)
    .execute(context.pool())
    .await
    .unwrap();
}
