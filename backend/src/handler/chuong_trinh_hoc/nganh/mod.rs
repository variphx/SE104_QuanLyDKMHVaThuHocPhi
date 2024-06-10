use axum::{extract::State, Json};

use crate::context::Context;

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct Nganh {
    id: String,
    id_khoa: String,
    ten: String,
}

pub async fn get(State(context): State<Context>) -> Result<Json<Vec<Nganh>>, ()> {
    let nganhs = sqlx::query_as::<_, Nganh>(
        "SELECT NGANH.id, NGANH.id_khoa, NGANH.ten FROM NGANH, KHOA
            WHERE NGANH.id_khoa = KHOA.id",
    )
    .fetch_all(context.pool())
    .await
    .unwrap();

    Ok(Json(nganhs))
}
