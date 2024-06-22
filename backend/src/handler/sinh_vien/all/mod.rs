use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router};
use serde::Serialize;

use crate::context::Context;

pub fn router() -> Router<Context> {
    Router::new().route("/get", axum::routing::get(get))
}

#[derive(Serialize, sqlx::FromRow)]
struct SinhVien {
    id: String,
    id_chuong_trinh_hoc: String,
    id_doi_tuong: String,
    id_gioi_tinh: String,
    id_que_quan: String,
    ngay_sinh: String,
    ten: String,
}

async fn get(State(context): State<Context>) -> impl IntoResponse {
    match sqlx::query_as::<_, SinhVien>(
        "select id, ten, to_char(ngay_sinh, 'yyyy-mm-dd') as ngay_sinh, id_gioi_tinh, id_que_quan, id_doi_tuong, id_chuong_trinh_hoc
            from sinh_vien"
    ).fetch_all(context.pool()).await {
        Ok(value)=> Json(value).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
    }
}
