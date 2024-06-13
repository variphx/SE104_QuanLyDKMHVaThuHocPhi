use axum::{extract::State, http::StatusCode, Json, Router};

use crate::context::Context;

#[derive(serde::Deserialize)]
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
) -> StatusCode {
    let id_hoc_ky = sqlx::query_scalar::<_, String>(
        "SELECT id_hoc_ky FROM THAM_SO
            WHERE id = 1",
    )
    .fetch_one(context.pool())
    .await
    .unwrap();

    let id_dang_ky_mon_hoc = sqlx::query_scalar::<_, String>(
        "SELECT id FROM DANG_KY_MON_HOC
            WHERE id_sinh_vien = $1
                AND id_hoc_ky = $2",
    )
    .bind(&payload.id_sinh_vien)
    .bind(&id_hoc_ky)
    .fetch_one(context.pool())
    .await
    .unwrap();

    sqlx::query(
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
    .unwrap();

    StatusCode::CREATED
}
