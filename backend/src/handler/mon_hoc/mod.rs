use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Serialize, sqlx::FromRow)]
struct MonHoc {
    id: String,
    id_khoa: String,
    ten: String,
    loai: String,
    so_tiet: i64,
}

#[derive(Deserialize)]
struct MonHocCreatePayload {
    id: String,
    id_khoa: String,
    ten: String,
    loai: String,
    so_tiet: String,
}

#[derive(Deserialize)]
struct MonHocQueryPayload {
    id: String,
}

#[derive(Deserialize)]
struct MonHocModifyPayload {
    id: String,
    payload: MonHocCreatePayload,
}

pub fn router() -> Router<Context> {
    Router::new()
        .route("/get", axum::routing::post(get))
        .route("/post", axum::routing::post(post))
        .route("/patch", axum::routing::post(patch))
        .route("/delete", axum::routing::post(delete))
}

async fn get(
    State(context): State<Context>,
    Json(payload): Json<MonHocQueryPayload>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, MonHoc>(
        "select * from mon_hoc
                where id = $1",
    )
    .bind(payload.id)
    .fetch_one(context.pool())
    .await
    {
        Ok(value) => Json(value).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", error)).into_response(),
    }
}

async fn post(
    State(context): State<Context>,
    Json(payload): Json<MonHocCreatePayload>,
) -> impl IntoResponse {
    let so_tiet = payload.so_tiet.parse::<i64>().unwrap();

    let so_tin_chi = {
        let he_so_tin_chi_lt = match sqlx::query_scalar::<_, i16>(
            "select he_so_tin_chi_lt from tham_so
            where id = 1",
        )
        .fetch_one(context.pool())
        .await
        {
            Ok(value) => value,
            Err(error) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", error)).into_response()
            }
        };

        let he_so_tin_chi_th = match sqlx::query_scalar::<_, i16>(
            "select he_so_tin_chi_th from tham_so
            where id = 1",
        )
        .fetch_one(context.pool())
        .await
        {
            Ok(value) => value,
            Err(error) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", error)).into_response()
            }
        };

        so_tiet
            / match payload.loai.as_str() {
                "LT" => he_so_tin_chi_lt,
                "TH" => he_so_tin_chi_th,
                _ => unreachable!(),
            } as i64
    };

    match sqlx::query(
        "insert into mon_hoc (id, id_khoa, ten, loai, so_tiet, so_tin_chi)
            values (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6
            )",
    )
    .bind(payload.id)
    .bind(payload.id_khoa)
    .bind(payload.ten)
    .bind(payload.loai)
    .bind(so_tiet)
    .bind(so_tin_chi as i16)
    .execute(context.pool())
    .await
    {
        Ok(_) => (StatusCode::CREATED).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", error)).into_response(),
    }
}

async fn patch(
    State(context): State<Context>,
    Json(MonHocModifyPayload { id, payload }): Json<MonHocModifyPayload>,
) {
    sqlx::query(
        "update mon_hoc
            set ten = $1,
                so_tiet = $2
            where id = $3",
    )
    .bind(payload.ten)
    .bind(payload.so_tiet)
    .bind(id)
    .execute(context.pool())
    .await
    .unwrap();
}

async fn delete(State(context): State<Context>, Json(payload): Json<MonHocQueryPayload>) {
    sqlx::query(
        "delete from mon_hoc
            where id = $1",
    )
    .bind(payload.id)
    .execute(context.pool())
    .await
    .unwrap();
}
