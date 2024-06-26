use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router};
use serde::{Deserialize, Serialize};

use crate::context::Context;

mod khoa;
mod nganh;

#[derive(Serialize, sqlx::FromRow)]
struct ChuongTrinhHoc {
    id: String,
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
        .route("/get/all", axum::routing::get(get_all))
        .route("/post", axum::routing::post(post))
        .nest("/nganh", nganh::router())
        .nest("/khoa", khoa::router())
}

async fn get(State(context): State<Context>, Json(id): Json<String>) -> impl IntoResponse {
    match sqlx::query_as::<_, ChuongTrinhHoc>(
        "select id, id_hoc_ky, id_nganh from chuong_trinh_hoc
            where id = $1",
    )
    .bind(&id)
    .fetch_one(context.pool())
    .await
    {
        Ok(x) => Json(x).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn get_all(State(context): State<Context>) -> impl IntoResponse {
    match sqlx::query_as::<_, ChuongTrinhHoc>(
        "select id, id_hoc_ky, id_nganh from chuong_trinh_hoc",
    )
    .fetch_all(context.pool())
    .await
    {
        Ok(x) => Json(x).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
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
