use axum::{extract::State, Json};
use serde::Deserialize;

use crate::context::Context;

#[derive(Deserialize)]
struct DoiTuongVungMienCreatePayload {
    id_thanh_pho: String,
    ten: String,
    mien_giam: f64,
}

pub fn method_router() -> axum::routing::MethodRouter<Context> {
    axum::routing::post(post)
}

async fn post(
    State(context): State<Context>,
    Json(payload): Json<DoiTuongVungMienCreatePayload>,
) -> Result<(), ()> {
    let id = {
        let doi_tuong_len = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT (*) FROM DOI_TUONG, DOI_TUONG_VUNG_MIEN
            WHERE DOI_TUONG.id = DOI_TUONG_VUNG_MIEN.id",
        )
        .fetch_one(context.pool())
        .await
        .unwrap();

        format!("VM{:03}", doi_tuong_len + 1)
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
        "INSERT INTO DOI_TUONG_VUNG_MIEN (id, id_thanh_pho)
            VALUES (
                $1,
                $2
            )",
    )
    .bind(id)
    .bind(payload.id_thanh_pho)
    .execute(context.pool())
    .await
    .unwrap();

    Ok(())
}
