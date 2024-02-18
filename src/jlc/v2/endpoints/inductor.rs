use axum::response::IntoResponse;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;
use axum::response::Response;

use crate::jlc::v2::models::*;
use crate::jlc::v2::services::dataframe_to_component;
use crate::jlc::v2::services::inductor::*;
use crate::AppState;

use self::inductor::InductorRequest;

/// JLC Inductor Part Request
#[utoipa::path(post, path = "/jlc/v2/inductor",
request_body = InductorRequest,
responses(
    (status = 200, description = "JLC Part Found", body = [Component]),
    (status = 400, description = "Bad Request", body = [NoPartFound]),
    (status = 404, description = "JLC Part Not Found", body = [NoPartFound])
)
)]
pub async fn part_request(
    State(AppState {
        ref inductor_df, ..
    }): State<AppState>,
    Json(payload): Json<InductorRequest>,
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
    let part_dataframe_option = find_inductor(inductor_df.clone(), payload);

    if part_dataframe_option.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(NoPartFound {
                code: 404,
                message: "No inductor found".to_string(),
            })
            .into_response(),
        );
    }

    let component = dataframe_to_component(part_dataframe_option.unwrap());

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::OK, Json(component).into_response())
}
