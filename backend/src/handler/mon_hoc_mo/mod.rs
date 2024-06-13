use axum::{extract::State, Json, Router};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Serialize, sqlx::FromRow)]
struct MonHocMo {
    id_mon_hoc: String,
    ten: String,
    loai: String,
    so_tin_chi: i32,
}

#[derive(Deserialize)]
struct MonHocMoQueryPayload {
    id_sinh_vien: String,
}

#[derive(Deserialize)]
struct MonHocMoCreatePayload {
    id_chuong_trinh_hoc: String,
    id_hoc_ky: String,
    id_mon_hoc: String,
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
    let mut mon_hoc_mos = sqlx::query_as::<_, MonHocMo>(
        "SELECT MON_HOC.id as id_mon_hoc, MON_HOC.ten, MON_HOC.loai, MON_HOC.so_tiet, 0 as so_tin_chi FROM MON_HOC_MO, SINH_VIEN, MON_HOC
            WHERE MON_HOC_MO.id_chuong_trinh_hoc = SINH_VIEN.id_chuong_trinh_hoc
                AND MON_HOC_MO.id_mon_hoc = MON_HOC.id
                AND SINH_VIEN.id = $1",
    )
    .bind(&payload.id_sinh_vien)
    .fetch_all(context.pool())
    .await
    .unwrap();

    Json(mon_hoc_mos)
}

async fn post(State(context): State<Context>, Json(payload): Json<MonHocMoCreatePayload>) {
    let id = format!(
        "{}{}{}",
        payload.id_hoc_ky, payload.id_chuong_trinh_hoc, payload.id_mon_hoc
    );

    sqlx::query(
        "INSERT INTO MON_HOC_MO (id, id_mon_hoc, id_chuong_trinh_hoc, id_hoc_ky)
            VALUES (
                $1,
                $2,
                $3,
                $4
            )",
    )
    .bind(id)
    .bind(payload.id_mon_hoc)
    .bind(payload.id_chuong_trinh_hoc)
    .bind(payload.id_hoc_ky)
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
