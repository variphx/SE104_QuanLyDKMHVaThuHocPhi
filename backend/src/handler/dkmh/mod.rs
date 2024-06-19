use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Deserialize)]
struct ChiTietDangKyMonHocCreatePayload {
    id_mon_hoc_mo: String,
    id_sinh_vien: String,
}

pub fn router() -> Router<Context> {
    Router::new()
        .route("/post", axum::routing::post(post))
        .route("/get", axum::routing::post(get))
}

async fn post(
    State(context): State<Context>,
    Json(ChiTietDangKyMonHocCreatePayload {
        id_mon_hoc_mo,
        id_sinh_vien,
    }): Json<ChiTietDangKyMonHocCreatePayload>,
) -> impl IntoResponse {
    match sqlx::query_scalar::<_, bool>(
        "select is_mo_dkmh from tham_so
            where id = 1",
    )
    .fetch_one(context.pool())
    .await
    {
        Ok(true) => (),
        Ok(false) => return (StatusCode::FORBIDDEN, "Chưa mở đăng kí học phần").into_response(),
        Err(error) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
        }
    };

    let id_hoc_ky = match sqlx::query_scalar::<_, String>(
        "select id_hoc_ky from tham_so
            where id = 1",
    )
    .fetch_one(context.pool())
    .await
    {
        Ok(value) => value,
        Err(error) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
        }
    };

    let id_dang_ky_mon_hoc = match sqlx::query_scalar::<_, String>(
        "select id from dang_ky_mon_hoc
            where id_sinh_vien = $1
                and id_hoc_ky = $2",
    )
    .bind(&id_sinh_vien)
    .bind(&id_hoc_ky)
    .fetch_one(context.pool())
    .await
    {
        Ok(value) => value,
        Err(error) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
        }
    };

    match sqlx::query(
        "insert into chi_tiet_dang_ky_mon_hoc (id_mon_hoc, id_dang_ky_mon_hoc)
            values (
                $1,
                $2
            )",
    )
    .bind(&id_mon_hoc_mo)
    .bind(&id_dang_ky_mon_hoc)
    .execute(context.pool())
    .await
    {
        Ok(_) => (StatusCode::CREATED).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

#[derive(Serialize, sqlx::FromRow)]
struct ChiTietDangKyMonHoc {
    id_mon_hoc: String,
}

async fn get(
    State(context): State<Context>,
    Json(id_sinh_vien): Json<String>,
) -> impl IntoResponse {
    match sqlx::query_scalar::<_, bool>(
        "select is_mo_dkmh from tham_so
            where id = 1",
    )
    .fetch_one(context.pool())
    .await
    {
        Ok(true) => (),
        Ok(false) => return (StatusCode::FORBIDDEN).into_response(),
        Err(error) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
        }
    };

    let id_hoc_ky = match sqlx::query_scalar::<_, String>(
        "select id_hoc_ky from tham_so
            where id = 1",
    )
    .fetch_one(context.pool())
    .await
    {
        Ok(value) => value,
        Err(error) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
        }
    };

    match sqlx::query_as::<_, ChiTietDangKyMonHoc>(
        "select id_mon_hoc from chi_tiet_dang_ky_mon_hoc
            where id_sinh_vien = $1
                and id_hoc_ky = $2",
    )
    .bind(id_sinh_vien)
    .bind(id_hoc_ky)
    .fetch_all(context.pool())
    .await
    {
        Ok(value) => Json(value).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}
