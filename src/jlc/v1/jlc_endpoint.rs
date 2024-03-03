use axum::response::IntoResponse;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;
use axum::response::Response;
use axum::Extension;
use sqlx::PgPool;

use super::jlc_models::*;
use super::jlc_part_finder::*;
use crate::AppState;

/// JLC Part Request
#[utoipa::path(post, path = "/jlc/v1",
request_body = JLCPartRequest,
responses(
    (status = 200, description = "JLC Part Found", body = [JLCPartResponse]),
    (status = 404, description = "JLC Part Not Found", body = [NoPartFound])
)
)]
pub async fn part_request(
    Extension(pool): Extension<PgPool>,
    State(state): State<AppState>,
    Json(payload): Json<JLCPartRequest>,
) -> (StatusCode, Response) {
    // insert your application logic here

    let part_response = find_part(pool, payload).await;

    if part_response.is_err() {
        return (
            StatusCode::NOT_FOUND,
            Json(NoPartFound {
                code: 404,
                message: part_response.unwrap_err(),
            })
            .into_response(),
        );
    }

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::OK, Json(part_response.unwrap()).into_response())
}
