use axum::response::IntoResponse;

use axum::http::StatusCode;
use axum::response::Json;
use axum::response::Response;
use axum::Extension;
use sqlx::PgPool;

use crate::jlc::v2::models::*;
use crate::jlc::v2::services::resistor::*;

use self::resistor::ResistorRequest;

/// JLC Resistor Part Request
#[utoipa::path(post, path = "/jlc/v2/resistor",
request_body = ResistorRequest,
responses(
    (status = 200, description = "JLC Part Found", body = [Component]),
    (status = 400, description = "Bad Request", body = [NoPartFound]),
    (status = 404, description = "JLC Part Not Found", body = [NoPartFound])
)
)]
pub async fn part_request(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<ResistorRequest>,
) -> (StatusCode, Response) {
    // validate the request
    if payload.tolerance_percentage.is_some() && payload.tolerance_percentage < Some(0.0)
        || payload.tolerance_percentage > Some(100.0)
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(NoPartFound {
                code: 400,
                message: "Tolerance percentage must be between 0 and 100".to_string(),
            })
            .into_response(),
        );
    }
    // validate that either absolute_tolerance and tolerance_percentage is set, not both and not neither
    if payload.absolute_tolerance.is_some() && payload.tolerance_percentage.is_some() {
        return (
            StatusCode::BAD_REQUEST,
            Json(NoPartFound {
                code: 400,
                message: "Either absolute_tolerance or tolerance_percentage must be set, not both"
                    .to_string(),
            })
            .into_response(),
        );
    }
    if payload.absolute_tolerance.is_none() && payload.tolerance_percentage.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(NoPartFound {
                code: 400,
                message:
                    "Either absolute_tolerance or tolerance_percentage must be set, not neither"
                        .to_string(),
            })
            .into_response(),
        );
    }
    // validate that if absolute_tolerance is set, absolute_tolerance_unit is also set
    if payload.absolute_tolerance.is_some() && payload.absolute_tolerance_unit.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(NoPartFound {
                code: 400,
                message: "If absolute_tolerance is set, absolute_tolerance_unit must also be set"
                    .to_string(),
            })
            .into_response(),
        );
    }

    // all is well, let's find the part
    let result_vec_component = find_resistor(pool, payload).await;

    if result_vec_component.is_err() {
        return (
            StatusCode::NOT_FOUND,
            Json(NoPartFound {
                code: 500,
                message: format!(
                    "Internal Server Error: {}",
                    result_vec_component.unwrap_err()
                ),
            })
            .into_response(),
        );
    }

    // unwrap the result and convert it into a JSON response
    // if the length of the vector is 0, return a 404
    let vec_component = result_vec_component.unwrap();
    if vec_component.len() == 0 {
        return (
            StatusCode::NOT_FOUND,
            Json(NoPartFound {
                code: 404,
                message: "No part found".to_string(),
            })
            .into_response(),
        );
    }
    // return the first element of the vector
    (
        StatusCode::OK,
        Json(vec_component.first().unwrap()).into_response(),
    )
}
