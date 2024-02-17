pub mod endpoints;
pub mod models;
pub mod services;

use axum::routing::post;
use axum::Router;

use crate::jlc::v2::endpoints::resistor::part_request;
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/resistor", post(part_request))
}
