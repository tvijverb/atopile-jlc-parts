pub mod jlc_endpoint;
pub mod jlc_models;
pub mod jlc_part_finder;
pub mod jlc_searchers;

use axum::routing::post;
use axum::Router;

use crate::jlc::v1::jlc_endpoint::part_request;
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(part_request))
}
