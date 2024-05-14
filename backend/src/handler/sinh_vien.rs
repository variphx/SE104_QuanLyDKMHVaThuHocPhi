use axum::{extract::State, http::StatusCode, Json};
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
struct SinhVienQueryPayload {
    ids: String,
}

pub fn method_router() -> axum::routing::MethodRouter<Context> {
    axum::routing::get(get)
        .post(post)
        .patch(patch)
        .delete(delete)
}

async fn get(
    State(context): State<Context>,
    Json(payload): Json<SinhVienQueryPayload>,
) -> Result<Json<Vec<SinhVien>>, StatusCode> {
    let mut sinh_viens = Vec::new();

    for id in payload.ids.split(',').map(str::trim) {
        sinh_viens.push(
            sqlx::query_as(
                "SELECT * FROM SINH_VIEN
                WHERE id = $1",
            )
            .bind(id)
            .fetch_one(context.pool())
            .await
            .unwrap(),
        );
    }

    Ok(Json(sinh_viens))
}

async fn post(
    State(context): State<Context>,
    Json(payload): Json<SinhVienCreatePayload>,
) -> Result<(), StatusCode> {
    let id = {
        let id_hoc_ky = sqlx::query_scalar::<_, String>(
            "SELECT HOC_KY.id FROM HOC_KY, CHUONG_TRINH_HOC
                WHERE HOC_KY.id = CHUONG_TRINH_HOC.id_hoc_ky
                    AND CHUONG_TRINH_HOC.id = $1",
        )
        .bind(&payload.id_chuong_trinh_hoc)
        .fetch_one(context.pool())
        .await
        .unwrap();

        let sinh_vien_len = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT (*) as len FROM SINH_VIEN, HOC_KY, CHUONG_TRINH_HOC
                WHERE SINH_VIEN.id_chuong_trinh_hoc = CHUONG_TRINH_HOC.id
                    AND CHUONG_TRINH_HOC.id_hoc_ky = $1",
        )
        .bind(&id_hoc_ky)
        .fetch_one(context.pool())
        .await
        .unwrap();

        format! {"{}{:04}", id_hoc_ky, sinh_vien_len+1}
    };

    let ngay_sinh = time::Date::parse(
        &payload.ngay_sinh,
        time::macros::format_description!("[year]-[month]-[day]"),
    )
    .map_err(|_| StatusCode::BAD_REQUEST)?;

    fun_name(id, payload, ngay_sinh, context).await?;

    Ok(())
}

async fn fun_name(
    id: String,
    payload: SinhVienCreatePayload,
    ngay_sinh: time::Date,
    context: Context,
) -> Result<(), StatusCode> {
    sqlx::query(
        "INSERT INTO SINH_VIEN (
            id,
            ten,
            can_cuoc,
            ngay_sinh,
            so_dien_thoai,
            email,
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
            $7,
            $8,
            $9,
            $10
        )",
    )
    .bind(&id)
    .bind(&payload.ten)
    .bind(&payload.can_cuoc)
    .bind(ngay_sinh)
    .bind(&payload.so_dien_thoai)
    .bind(&payload.email)
    .bind(&payload.id_gioi_tinh)
    .bind(&payload.id_que_quan)
    .bind(&payload.id_doi_tuong)
    .bind(&payload.id_chuong_trinh_hoc)
    .execute(context.pool())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

async fn patch(State(context): State<Context>, Json(payload): Json<SinhVienModifyPayload>) {}
async fn delete() {}
