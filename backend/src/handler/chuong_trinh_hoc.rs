use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Serialize, sqlx::FromRow)]
struct ChuongTrinhHoc {
    id: String,
    id_nganh: String,
    id_hoc_ky: String,
}

#[derive(Deserialize)]
struct ChuongTrinhHocCreatePayload {
    id_nganh: String,
    id_hoc_ky: String,
}

#[derive(Deserialize)]
struct ChuongTrinhHocQueryPayload {
    id: String,
}

pub fn method_router() -> axum::routing::MethodRouter<Context> {
    axum::routing::get(get).post(post)
}

async fn get(
    State(context): State<Context>,
    Json(payload): Json<ChuongTrinhHocQueryPayload>,
) -> Result<Json<ChuongTrinhHoc>, StatusCode> {
    let chuong_trinh_hoc = sqlx::query_as::<_, ChuongTrinhHoc>(
        "SELECT * FROM CHUONG_TRINH_HOC
            WHERE id = $1",
    )
    .bind(&payload.id)
    .fetch_one(context.pool())
    .await
    .unwrap();

    Ok(Json(chuong_trinh_hoc))
}

async fn post(
    State(context): State<Context>,
    Json(payload): Json<ChuongTrinhHocCreatePayload>,
) -> Result<(), StatusCode> {
    let id = format!("{}{}", payload.id_nganh, payload.id_hoc_ky);
    sqlx::query(
        "INSERT INTO CHUONG_TRINH_HOC (id, id_nganh, id_hoc_ky)
            VALUES (
                $1,
                $2,
                $3
            )",
    )
    .bind(id)
    .bind(payload.id_nganh)
    .bind(payload.id_hoc_ky)
    .execute(context.pool())
    .await
    .unwrap();

    Ok(())
}
