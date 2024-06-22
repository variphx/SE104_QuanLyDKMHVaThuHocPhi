use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

use crate::context::Context;

mod all;
mod chua_dong_hoc_phi;

#[derive(Serialize, sqlx::FromRow)]
struct SinhVien {
    ten: String,
    ngay_sinh: String,
    id_gioi_tinh: String,
    id_que_quan: String,
    id_doi_tuong: String,
    id_chuong_trinh_hoc: String,
}

#[derive(Deserialize)]
struct SinhVienCreatePayload {
    ten: String,
    ngay_sinh: String,
    id_gioi_tinh: String,
    id_que_quan: String,
    id_doi_tuong: String,
    id_nganh: String,
}

#[derive(Deserialize)]
struct SinhVienModifyPayload {
    id: String,
    payload: SinhVienCreatePayload,
}

pub fn router() -> Router<Context> {
    Router::new()
        .route("/get", axum::routing::post(get))
        .route("/post", axum::routing::post(post))
        .route("/patch", axum::routing::post(patch))
        .route("/delete", axum::routing::post(delete))
        .route(
            "/chua-dong-hoc-phi/get",
            axum::routing::get(chua_dong_hoc_phi::get),
        )
        .nest("/all", all::router())
}

async fn get(State(context): State<Context>, Json(id): Json<String>) -> Json<SinhVien> {
    let sinh_vien = sqlx::query_as::<_, SinhVien>(
        "select ten, to_char(ngay_sinh, 'yyyy-mm-dd') as ngay_sinh, id_gioi_tinh, id_que_quan, id_doi_tuong, id_chuong_trinh_hoc from sinh_vien
            where id = $1",
    )
    .bind(id)
    .fetch_one(context.pool())
    .await
    .unwrap();

    Json(sinh_vien)
}

async fn post(
    State(context): State<Context>,
    Json(SinhVienCreatePayload {
        ten,
        ngay_sinh,
        id_gioi_tinh,
        id_que_quan,
        id_doi_tuong,
        id_nganh,
    }): Json<SinhVienCreatePayload>,
) -> impl IntoResponse {
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

    let id_chuong_trinh_hoc = {
        match sqlx::query_scalar::<_, String>(
            "select id from chuong_trinh_hoc
                where id_nganh = $1
                    and id_hoc_ky = $2",
        )
        .bind(id_nganh)
        .bind(&id_hoc_ky)
        .fetch_one(context.pool())
        .await
        {
            Ok(value) => value,
            Err(error) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
            }
        }
    };

    let id = {
        let sinh_vien_len = match sqlx::query_scalar::<_, i64>(
            "select count (*) from sinh_vien, chuong_trinh_hoc
                where sinh_vien.id_chuong_trinh_hoc = chuong_trinh_hoc.id
                    and chuong_trinh_hoc.id_hoc_ky = $1",
        )
        .bind(&id_hoc_ky)
        .fetch_one(context.pool())
        .await
        {
            Ok(value) => value,
            Err(error) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
            }
        };

        format!("{}{:04}", id_hoc_ky, sinh_vien_len + 1)
    };

    match sqlx::query(
        "insert into sinh_vien (
            id,
            ten,
            ngay_sinh,
            id_gioi_tinh,
            id_que_quan,
            id_doi_tuong,
            id_chuong_trinh_hoc
        )
        values (
            $1,
            $2,
            to_date($3, 'yyyy-mm-dd'),
            $4,
            $5,
            $6,
            $7
        )",
    )
    .bind(&id)
    .bind(ten)
    .bind(ngay_sinh)
    .bind(id_gioi_tinh)
    .bind(id_que_quan)
    .bind(id_doi_tuong)
    .bind(id_chuong_trinh_hoc)
    .execute(context.pool())
    .await
    {
        Ok(_) => (),
        Err(error) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
        }
    };

    let password = id.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = match argon2.hash_password(password, &salt) {
        Ok(value) => value,
        Err(error) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
        }
    }
    .to_string();

    match sqlx::query(
        "insert into users (username, password)
            values (
                $1,
                $2
            )",
    )
    .bind(id.as_str())
    .bind(password_hash.as_str())
    .execute(context.pool())
    .await
    {
        Ok(_) => (StatusCode::CREATED).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}

async fn patch(
    State(context): State<Context>,
    Json(SinhVienModifyPayload { id, payload }): Json<SinhVienModifyPayload>,
) {
    sqlx::query(
        "DELETE FROM SINH_VIEN
            WHERE id = $1",
    )
    .bind(&id)
    .execute(context.pool())
    .await
    .unwrap();

    let ngay_sinh = time::Date::parse(
        &payload.ngay_sinh,
        time::macros::format_description!("[year]-[month]-[day]"),
    )
    .unwrap();

    sqlx::query(
        "INSERT INTO SINH_VIEN (
            id,
            ten,
            ngay_sinh,
            id_gioi_tinh,
            id_que_quan,
            id_doi_tuong,
            id_chuong_trinh_hoc
        )
        VALUES (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7
        )",
    )
    .bind(id)
    .bind(&payload.ten)
    .bind(ngay_sinh)
    .bind(&payload.id_gioi_tinh)
    .bind(&payload.id_que_quan)
    .bind(&payload.id_doi_tuong)
    .bind(&payload.id_nganh)
    .execute(context.pool())
    .await
    .unwrap();
}

async fn delete(State(context): State<Context>, Json(id): Json<String>) {
    sqlx::query(
        "DELETE FROM SINH_VIEN
            WHERE id = $1",
    )
    .bind(id)
    .execute(context.pool())
    .await
    .unwrap();
}
