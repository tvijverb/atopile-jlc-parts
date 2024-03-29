pub mod endpoints;
pub mod models;
pub mod services;

use axum::routing::post;
use axum::Router;

use crate::jlc::v2::endpoints::capacitor;
use crate::jlc::v2::endpoints::inductor;
use crate::jlc::v2::endpoints::resistor;
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/resistor", post(resistor::part_request))
        .route("/capacitor", post(capacitor::part_request))
        .route("/inductor", post(inductor::part_request))
}
