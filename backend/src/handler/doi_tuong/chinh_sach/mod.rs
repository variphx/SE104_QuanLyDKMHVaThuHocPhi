use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
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
) -> Result<StatusCode, StatusCode> {
    let id = {
        let doi_tuong_len = match sqlx::query_scalar::<_, i64>(
            "SELECT COUNT (*) FROM DOI_TUONG, DOI_TUONG_CHINH_SACH
                WHERE DOI_TUONG.id = DOI_TUONG_CHINH_SACH.id",
        )
        .fetch_one(context.pool())
        .await
        {
            Ok(value) => value,
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)?,
        };

        format!("DTCS{:03}", doi_tuong_len + 1)
    };

    match sqlx::query(
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
    {
        Ok(_) => {}
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)?,
    };

    match sqlx::query(
        "INSERT INTO DOI_TUONG_CHINH_SACH (id)
                VALUES (
                    $1
                )",
    )
    .bind(id)
    .execute(context.pool())
    .await
    {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(serde::Serialize, sqlx::FromRow)]
struct DoiTuong {
    id: String,
    ten: String,
}

pub async fn get_all(State(context): State<Context>) -> impl IntoResponse {
    match sqlx::query_as::<_, DoiTuong>(
        "select doi_tuong.id, doi_tuong.ten from doi_tuong, doi_tuong_chinh_sach
            where doi_tuong.id = doi_tuong_chinh_sach.id",
    )
    .fetch_all(context.pool())
    .await
    {
        Ok(x) => Json(x).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
