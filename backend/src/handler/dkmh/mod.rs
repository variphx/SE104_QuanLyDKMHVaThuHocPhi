use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router};
use serde::Deserialize;

use crate::context::Context;

#[derive(Deserialize)]
struct ChiTietDangKyMonHocCreatePayload {
    id_mon_hoc: String,
    id_sinh_vien: String,
}

pub fn router() -> Router<Context> {
    Router::new().route("/post", axum::routing::post(post))
}

async fn post(
    State(context): State<Context>,
    Json(payload): Json<ChiTietDangKyMonHocCreatePayload>,
) -> impl IntoResponse {
    let id_hoc_ky = match sqlx::query_scalar::<_, String>(
        "SELECT id_hoc_ky FROM THAM_SO
            WHERE id = 1",
    )
    .fetch_one(context.pool())
    .await
    {
        Ok(value) => value,
        Err(error) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
        }
    };

    let id_dang_ky_mon_hoc = match sqlx::query_scalar::<_, String>(
        "SELECT id FROM DANG_KY_MON_HOC
            WHERE id_sinh_vien = $1
                AND id_hoc_ky = $2",
    )
    .bind(&payload.id_sinh_vien)
    .bind(&id_hoc_ky)
    .fetch_one(context.pool())
    .await
    {
        Ok(value) => value,
        Err(error) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
        }
    };

    match sqlx::query(
        "INSERT INTO CHI_TIET_DANG_KY_MON_HOC (id_mon_hoc, id_dang_ky_mon_hoc)
            VALUES (
                $1,
                $2
            )",
    )
    .bind(&payload.id_mon_hoc)
    .bind(&id_dang_ky_mon_hoc)
    .execute(context.pool())
    .await
    {
        Ok(_) => (StatusCode::CREATED).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

#[derive(Deserialize)]
struct ChiTietDangKyMonHocQueryPayload {
    id_sinh_vien: String,
    id_hoc_ky: String,
}

struct ChiTietDangKyMonHoc {
    id_mon_hoc: String,
}

async fn get(
    State(context): State<Context>,
    Json(ChiTietDangKyMonHocQueryPayload {
        id_sinh_vien,
        id_hoc_ky,
    }): Json<ChiTietDangKyMonHocQueryPayload>,
) -> impl IntoResponse {
    todo!()
}
