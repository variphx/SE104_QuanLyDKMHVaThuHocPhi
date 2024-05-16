use axum::{extract::State, Json};
use serde::Deserialize;

use crate::context::Context;

struct MonHocMo {
    id_mon_hoc: String,
    id_chuong_trinh_hoc: String,
}

#[derive(Deserialize)]
struct MonHocMoQueryPayload {}

#[derive(Deserialize)]
struct MonHocMoCreatePayload {
    id_mon_hoc: String,
    id_chuong_trinh_hoc: String,
}

pub fn method_router() -> axum::routing::MethodRouter<Context> {
    axum::routing::get(get).post(post).delete(delete)
}

async fn get(State(context): State<Context>, Json(payload): Json<MonHocMoQueryPayload>) {}

async fn post(State(context): State<Context>, Json(payload): Json<MonHocMoCreatePayload>) {
    sqlx::query(
        "INSERT INTO MON_HOC_MO (id_mon_hoc, id_chuong_trinh_hoc)
            VALUES (
                $1,
                $2
            )",
    )
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
