pub mod endpoints;
pub mod models;
pub mod services;

use axum::routing::post;
use axum::Router;

use crate::jlc::v2::endpoints::resistor;
use crate::jlc::v2::endpoints::capacitor;
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
    .route("/resistor", post(resistor::part_request))
    .route("/capacitor", post(capacitor::part_request))
}
