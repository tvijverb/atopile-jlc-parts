use axum::response::IntoResponse;
use axum::routing::post;
use axum::{http::StatusCode, Router};
use axum::response::Json;
use axum::response::Response;

use crate::jlc_models::*;
use crate::jlc_part_finder::*;

/// JLC Part Request
#[utoipa::path(post, path = "/jlc",
request_body = JLCPartRequest,
responses(
    (status = 200, description = "JLC Part Found", body = [JLCPartResponse]),
    (status = 404, description = "JLC Part Not Found", body = [NoPartFound])
)
)]
async fn part_request(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<JLCPartRequest>,
) -> (StatusCode, Response) {
    // insert your application logic here
    
    let part_response = find_part(payload);

    if part_response.is_err() {
        return (StatusCode::NOT_FOUND, Json(NoPartFound {code: 404, message: part_response.unwrap_err() } ).into_response());
    }

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::OK, Json(part_response.unwrap()).into_response())
}

pub fn router() -> Router {
    Router::new().route("/", post(part_request))
}
