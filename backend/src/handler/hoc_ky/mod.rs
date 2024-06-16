use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router};
use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Serialize, sqlx::FromRow)]
struct HocKy {
    ten: String,
    nam_hoc: i32,
}

#[derive(Deserialize)]
struct HocKyCreatePayload {
    ten: String,
    nam_hoc: i64,
}

pub fn router() -> Router<Context> {
    Router::new()
        .route("/get", axum::routing::post(get))
        .route("/post", axum::routing::post(post))
}

async fn get(State(context): State<Context>, Json(id): Json<String>) -> impl IntoResponse {
    match sqlx::query_as::<_, HocKy>(
        "SELECT ten, nam_hoc FROM HOC_KY
                WHERE id = $1",
    )
    .bind(id)
    .fetch_one(context.pool())
    .await
    {
        Ok(value) => Json(value).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
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
