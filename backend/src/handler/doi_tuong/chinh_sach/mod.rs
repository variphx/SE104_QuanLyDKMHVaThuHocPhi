use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;

use crate::context::Context;

#[derive(Deserialize)]
pub struct DoiTuongChinhSachCreatePayload {
    ten: String,
    mien_giam: String,
}

pub async fn post(
    State(context): State<Context>,
    Json(payload): Json<DoiTuongChinhSachCreatePayload>,
) -> Result<(), StatusCode> {
    let id = {
        let doi_tuong_len = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT (*) FROM DOI_TUONG, DOI_TUONG_CHINH_SACH
                WHERE DOI_TUONG.id = DOI_TUONG_CHINH_SACH.id",
        )
        .fetch_one(context.pool())
        .await
        .unwrap();

        format!("DTCS{:03}", doi_tuong_len + 1)
    };

    sqlx::query(
        "INSERT INTO DOI_TUONG (id, ten, mien_giam)
            VALUES (
                $1,
                $2,
                $3
            )",
    )
    .bind(&id)
    .bind(payload.ten)
    .bind(payload.mien_giam)
    .execute(context.pool())
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO DOI_TUONG_CHINH_SACH (id)
            VALUES (
                $1
            )",
    )
    .bind(id)
    .execute(context.pool())
    .await
    .unwrap();

    Ok(())
}
