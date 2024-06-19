use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router};
use serde::{Deserialize, Serialize};

use crate::context::Context;

mod khoa;
mod nganh;

#[derive(Serialize, sqlx::FromRow)]
struct ChuongTrinhHoc {
    id_hoc_ky: String,
    id_nganh: String,
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
        .route("/nganh/get", axum::routing::post(nganh::get))
        .route("/khoa/get", axum::routing::post(khoa::get))
}

async fn get(State(context): State<Context>, Json(id): Json<String>) -> impl IntoResponse {
    match sqlx::query_as::<_, ChuongTrinhHoc>(
        "select id_hoc_ky, id_nganh from chuong_trinh_hoc
            where id = $1",
    )
    .bind(&id)
    .fetch_one(context.pool())
    .await
    {
        Ok(chuong_trinh_hoc) => Json(chuong_trinh_hoc).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}

async fn post(
    State(context): State<Context>,
    Json(payload): Json<ChuongTrinhHocCreatePayload>,
) -> impl IntoResponse {
    match sqlx::query(
        "insert into chuong_trinh_hoc (id_nganh, id_hoc_ky)
            values (
                $1,
                $2
            )",
    )
    .bind(payload.id_nganh)
    .bind(payload.id_hoc_ky)
    .execute(context.pool())
    .await
    {
        Ok(_) => (StatusCode::CREATED).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}
