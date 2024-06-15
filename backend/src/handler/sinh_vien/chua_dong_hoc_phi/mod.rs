use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

use crate::context::Context;

#[derive(Serialize, sqlx::FromRow)]
struct SinhVienChuaDongHocPhi {
    ten: String,
    id: String,
    tong: f64,
    da_thanh_toan: f64,
}

pub async fn get(State(context): State<Context>) -> impl IntoResponse {
    match sqlx::query_as::<_, SinhVienChuaDongHocPhi>(
            "SELECT SINH_VIEN.id, SINH_VIEN.ten, SUM (HOC_PHI.tong) as tong, SUM (HOC_PHI.da_thanh_toan) as da_thanh_toan
                FROM SINH_VIEN, HOC_PHI
                WHERE SINH_VIEN.id = HOC_PHI.id_sinh_vien
                    AND HOC_PHI.tong <> HOC_PHI.da_thanh_toan
                GROUP BY SINH_VIEN.id, SINH_VIEN.ten"
        ).fetch_all(context.pool()).await {
        Ok(value) => {Json(value).into_response()},
        Err(error) => { (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", error)).into_response()},
    }
}
