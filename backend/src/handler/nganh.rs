use serde::Serialize;

use crate::context::Context;

#[derive(Serialize)]
struct Nganh {
    id: String,
    ten: String,
}

pub fn method_router() -> axum::routing::MethodRouter<Context> {
    todo!()
}
