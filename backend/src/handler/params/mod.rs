use axum::Router;

use crate::context::Context;

mod hoc_ky;

pub fn router() -> Router<Context> {
    Router::new().route("/current-hoc-ky/get", axum::routing::get(hoc_ky::get))
}
