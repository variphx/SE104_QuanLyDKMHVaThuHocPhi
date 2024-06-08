use axum::{extract::State, http::StatusCode, Json, Router};
use serde::{Deserialize, Serialize};

use crate::context::Context;

pub mod chinh_sach;
pub mod vung_mien;

#[derive(Serialize, sqlx::FromRow)]
struct DoiTuong {
    id: String,
    id_loai_doi_tuong: String,
    ten: String,
    mien_giam: f64,
}

#[derive(Deserialize)]
struct DoiTuongQueryPayload {
    id: String,
}

#[derive(Deserialize)]
struct DoiTuongModifyPayload {
    id: String,
    ten: Option<String>,
    mien_giam: Option<f64>,
}

pub fn router() -> Router<Context> {
    Router::new()
        .route("/get", axum::routing::post(get))
        .route("/patch", axum::routing::post(patch))
        .route("/delete", axum::routing::post(delete))
        .route("/chinh-sach/post", axum::routing::post(chinh_sach::post))
        .route("/vung-mien/post", axum::routing::post(vung_mien::post))
}

async fn get(
    State(context): State<Context>,
    Json(payload): Json<DoiTuongQueryPayload>,
) -> Result<Json<DoiTuong>, StatusCode> {
    let doi_tuong = sqlx::query_as::<_, DoiTuong>(
        "SELECT * FROM DOI_TUONG
            WHERE id = $1",
    )
    .bind(payload.id)
    .fetch_one(context.pool())
    .await
    .unwrap();

    Ok(Json(doi_tuong))
}

async fn patch(
    State(context): State<Context>,
    Json(payload): Json<DoiTuongModifyPayload>,
) -> Result<(), StatusCode> {
    let DoiTuongModifyPayload { id, ten, mien_giam } = payload;

    if let Some(ten) = ten {
        sqlx::query(
            "UPDATE DOI_TUONG
                SET ten = $1
                WHERE id = $2",
        )
        .bind(ten)
        .bind(&id)
        .execute(context.pool())
        .await
        .unwrap();
    }

    if let Some(mien_giam) = mien_giam {
        sqlx::query(
            "UPDATE DOI_TUONG
                SET mien_giam = $1
                WHERE id = $2",
        )
        .bind(mien_giam)
        .bind(id)
        .execute(context.pool())
        .await
        .unwrap();
    }

    Ok(())
}

async fn delete(
    State(context): State<Context>,
    Json(DoiTuongQueryPayload { id }): Json<DoiTuongQueryPayload>,
) -> Result<(), StatusCode> {
    sqlx::query(
        "DELETE FROM DOI_TUONG
            WHERE id = $1",
    )
    .bind(&id)
    .execute(context.pool())
    .await
    .unwrap();

    sqlx::query(
        "DELETE FROM DOI_TUONG_CHINH_SACH
            WHERE id = $1",
    )
    .bind(&id)
    .execute(context.pool())
    .await
    .unwrap();

    sqlx::query(
        "DELETE FROM DOI_TUONG_VUNG_MIEN
            WHERE id = $1",
    )
    .bind(&id)
    .execute(context.pool())
    .await
    .unwrap();

    Ok(())
}
