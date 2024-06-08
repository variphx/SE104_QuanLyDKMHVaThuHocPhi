use axum::{extract::State, Json, Router};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Serialize, sqlx::FromRow)]
struct MonHocMo {
    id_mon_hoc: String,
    id_chuong_trinh_hoc: String,
}

#[derive(Deserialize)]
struct MonHocMoQueryPayload {
    id_sinh_vien: String,
}

#[derive(Deserialize)]
struct MonHocMoCreatePayload {
    id_mon_hoc: String,
    id_chuong_trinh_hoc: String,
}

pub fn router() -> Router<Context> {
    Router::new()
        .route("/get", axum::routing::post(get))
        .route("/post", axum::routing::post(post))
        .route("/delete", axum::routing::post(delete))
}

async fn get(
    State(context): State<Context>,
    Json(payload): Json<MonHocMoQueryPayload>,
) -> Json<Vec<MonHocMo>> {
    let mon_hoc_mos = sqlx::query_as::<_, MonHocMo>(
        "SELECT MON_HOC_MO.id_mon_hoc, MON_HOC_MO.id_chuong_trinh_hoc FROM MON_HOC_MO, SINH_VIEN
            WHERE MON_HOC_MO.id_chuong_trinh_hoc = SINH_VIEN.id_chuong_trinh_hoc
                AND SINH_VIEN.id = $1",
    )
    .bind(&payload.id_sinh_vien)
    .fetch_all(context.pool())
    .await
    .unwrap();

    Json(mon_hoc_mos)
}

async fn post(State(context): State<Context>, Json(payload): Json<MonHocMoCreatePayload>) {
    let id = format!("{}{}", payload.id_chuong_trinh_hoc, payload.id_mon_hoc);

    sqlx::query(
        "INSERT INTO MON_HOC_MO (id, id_mon_hoc, id_chuong_trinh_hoc)
            VALUES (
                $1,
                $2,
                $3
            )",
    )
    .bind(id)
    .bind(payload.id_mon_hoc)
    .bind(payload.id_chuong_trinh_hoc)
    .execute(context.pool())
    .await
    .unwrap();
}

async fn delete(State(context): State<Context>, Json(payload): Json<MonHocMoCreatePayload>) {
    sqlx::query(
        "DELETE FROM MON_HOC_MO
            WHERE id_mon_hoc = $1
                AND id_chuong_trinh_hoc = $2",
    )
    .bind(payload.id_mon_hoc)
    .bind(payload.id_chuong_trinh_hoc)
    .execute(context.pool())
    .await
    .unwrap();
}
