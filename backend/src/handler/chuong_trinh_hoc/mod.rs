use axum::{extract::State, http::StatusCode, Json, Router};
use serde::{Deserialize, Serialize};

use crate::context::Context;

mod nganh;

#[derive(Serialize, sqlx::FromRow)]
struct ChuongTrinhHoc {
    id: String,
    id_nganh: String,
    id_hoc_ky: String,
}

#[derive(Deserialize)]
struct ChuongTrinhHocQueryPayload {
    id: String,
}

pub fn router() -> Router<Context> {
    Router::new()
        .route("/get", axum::routing::post(get))
        .route("/nganh/get", axum::routing::get(nganh::get))
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
