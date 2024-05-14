use serde::{Deserialize, Serialize};

use crate::context::Context;

#[derive(Serialize)]
struct QueQuan {
    thanh_pho: String,
    tinh: String,
}

#[derive(Deserialize)]
struct QueQuanCreatePayload {
    thanh_pho: String,
    tinh: String,
}

pub fn method_router() -> axum::routing::MethodRouter<Context> {
    todo!()
}
