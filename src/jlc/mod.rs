pub mod v1;
pub mod v2;

use axum::Router;

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().nest("/v1", v1::router()).nest("/v2", v2::router())
}
