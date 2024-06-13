use axum::{body::Body, extract::State, http::StatusCode, response::IntoResponse, Json, Router};
use serde::{Deserialize, Serialize};

use crate::context::Context;

mod nganh;

#[derive(Serialize, sqlx::FromRow)]
struct ChuongTrinhHoc {
    id: String,
    id_hoc_ky: String,
    id_nganh: String,
}

#[derive(Deserialize)]
struct ChuongTrinhHocQueryPayload {
    id: String,
}

#[derive(Deserialize)]
struct ChuongTrinhHocCreatePayload {
    id_hoc_ky: String,
    id_nganh: String,
}

pub fn router() -> Router<Context> {
    Router::new()
        .route("/get", axum::routing::post(get))
        .route("/post", axum::routing::post(post))
        .route("/nganh/get", axum::routing::get(nganh::get))
}

async fn get(
    State(context): State<Context>,
    Json(payload): Json<ChuongTrinhHocQueryPayload>,
) -> Result<Json<ChuongTrinhHoc>, StatusCode> {
    let chuong_trinh_hoc = match sqlx::query_as::<_, ChuongTrinhHoc>(
        "SELECT * FROM CHUONG_TRINH_HOC
            WHERE id = $1",
    )
    .bind(&payload.id)
    .fetch_one(context.pool())
    .await
    {
        Ok(chuong_trinh_hoc) => chuong_trinh_hoc,
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)?,
    };

    Ok(Json(chuong_trinh_hoc))
}

async fn post(
    State(context): State<Context>,
    Json(payload): Json<ChuongTrinhHocCreatePayload>,
) -> Result<StatusCode, StatusCode> {
    let id = format!("{}{}", payload.id_nganh, payload.id_hoc_ky);

    match sqlx::query(
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
    {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
