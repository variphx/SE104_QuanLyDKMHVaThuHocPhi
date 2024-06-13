use axum::{extract::State, http::StatusCode, Json};

use crate::context::Context;

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct Nganh {
    id: String,
    id_khoa: String,
    ten: String,
}

pub async fn get(State(context): State<Context>) -> Result<Json<Vec<Nganh>>, StatusCode> {
    let nganhs = match sqlx::query_as::<_, Nganh>(
        "SELECT NGANH.id, NGANH.id_khoa, NGANH.ten FROM NGANH, KHOA
            WHERE NGANH.id_khoa = KHOA.id",
    )
    .fetch_all(context.pool())
    .await
    {
        Ok(nganhs) => nganhs,
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)?,
    };

    Ok(Json(nganhs))
}
