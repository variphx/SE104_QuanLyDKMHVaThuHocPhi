use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router};
use serde::Serialize;

use crate::context::Context;

mod thanh_pho;
mod tinh;

#[derive(Serialize, sqlx::FromRow)]
struct QueQuan {
    id: String,
    thanh_pho: String,
    tinh: String,
}

pub fn router() -> Router<Context> {
    Router::new()
        .route("/get", axum::routing::post(get))
        .route("/get/all", axum::routing::get(get_all))
        .nest("/thanh-pho", thanh_pho::router())
        .nest("/tinh", tinh::router())
}

async fn get(State(context): State<Context>, Json(id): Json<String>) -> impl IntoResponse {
    match sqlx::query_as::<_, QueQuan>(
        "select thanh_pho.id, thanh_pho.ten as thanh_pho, tinh.ten as tinh from thanh_pho, tinh
            where thanh_pho.id_tinh = tinh.id
                and thanh_pho.id = $1",
    )
    .bind(id)
    .fetch_one(context.pool())
    .await
    {
        Ok(value) => Json(value).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}

async fn get_all(State(context): State<Context>) -> impl IntoResponse {
    match sqlx::query_as::<_, QueQuan>(
        "select thanh_pho.id, thanh_pho.ten as thanh_pho, tinh.ten as tinh from thanh_pho, tinh
            where thanh_pho.id_tinh = tinh.id",
    )
    .fetch_all(context.pool())
    .await
    {
        Ok(x) => Json(x).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
