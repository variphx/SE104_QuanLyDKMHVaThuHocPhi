use axum::{extract::State, Json, Router};
use serde::Serialize;

use crate::context::Context;

#[derive(Serialize, sqlx::FromRow)]
struct QueQuan {
    id: String,
    thanh_pho: String,
    tinh: String,
}

pub fn router() -> Router<Context> {
    Router::new().route("/get", axum::routing::post(get))
}

async fn get(State(context): State<Context>) -> Result<Json<Vec<QueQuan>>, Json<String>> {
    let que_quans = sqlx::query_as::<_, QueQuan>(
        "SELECT THANH_PHO.id, THANH_PHO.ten as thanh_pho, TINH.ten as tinh FROM THANH_PHO, TINH
            WHERE THANH_PHO.id_tinh = TINH.id",
    )
    .fetch_all(context.pool())
    .await
    .unwrap();

    Ok(Json(que_quans))
}
