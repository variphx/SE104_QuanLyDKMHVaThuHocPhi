use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Serialize, sqlx::FromRow)]
struct HocKy {
    id: String,
    ten: String,
    nam: i32,
}

#[derive(Deserialize)]
struct HocKyQueryPayload {
    id: String,
}

#[derive(Deserialize)]
struct HocKyCreatePayload {
    ten: String,
    nam_hoc: i64,
}

struct HocKyModifyPayload {
    id: String,
    ten: Option<String>,
    nam_hoc: Option<i64>,
}

pub fn method_router() -> axum::routing::MethodRouter<Context> {
    axum::routing::get(get).post(post).delete(delete)
}

async fn get(
    State(context): State<Context>,
    Json(HocKyQueryPayload { id }): Json<HocKyQueryPayload>,
) -> Result<Json<HocKy>, StatusCode> {
    let hoc_ky = sqlx::query_as::<_, HocKy>(
        "SELECT * FROM HOC_KY
            WHERE id = $1",
    )
    .bind(id)
    .fetch_one(context.pool())
    .await
    .unwrap();

    Ok(Json(hoc_ky))
}

async fn post(
    State(context): State<Context>,
    Json(HocKyCreatePayload { ten, nam_hoc }): Json<HocKyCreatePayload>,
) -> Result<(), ()> {
    let id = format!("{:04}{:02}", nam_hoc, ten);

    sqlx::query(
        "INSERT INTO HOC_KY (id, ten, nam_hoc)
            VALUES (
                $1,
                $2,
                $3
            )",
    )
    .bind(id)
    .bind(ten)
    .bind(nam_hoc)
    .execute(context.pool())
    .await
    .unwrap();

    Ok(())
}

async fn delete(
    State(context): State<Context>,
    Json(HocKyQueryPayload { id }): Json<HocKyQueryPayload>,
) -> Result<(), ()> {
    sqlx::query(
        "DELETE FROM HOC_KY
            WHERE id = $1",
    )
    .bind(id)
    .execute(context.pool())
    .await
    .unwrap();

    Ok(())
}
