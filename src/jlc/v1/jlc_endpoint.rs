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
    State(_state): State<AppState>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Args;

    use ::axum_test::TestServer;
    use ::serde_json::json;
    use axum::routing::post;
    use axum::Router;
    use clap::Parser;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_part_request() {
        dotenv().ok();
        let args = Args::parse();
        let pool = PgPool::connect(args.database_url.as_str()).await.unwrap();

        let app_state = AppState {};

        let app = Router::new()
            .route("/jlc/v1", post(part_request))
            .layer(Extension(pool))
            .with_state(app_state);

        let server = TestServer::new(app).unwrap();

        let response = server.post("/jlc/v1")
            .json(&json!({"designator_prefix": "R", "mpn": "generic_resistor", "type": "resistor", "value": {"unit": "megaohm", "min_val": 5.01534, "max_val": 5.1166599999999995, "nominal": 5.0663}, "package": "0402"}))
            .await;
        assert_eq!(response.status_code(), StatusCode::OK);

        let response = server.post("/jlc/v1")
        .json(&json!({"designator_prefix": "R", "mpn": "generic_resistor", "type": "resistor", "value": {"unit": "kiloohm", "min_val": 0.95, "max_val": 1.05, "nominal": 1}}))
        .await;
        assert_eq!(response.status_code(), StatusCode::OK);

        let response = server.post("/jlc/v1")
        .json(&json!({"designator_prefix": "C", "mpn": "generic_capacitor", "type": "capacitor", "value": {"unit": "nanofarad", "min_val": 80.0, "max_val": 120.0, "nominal": 100.0}, "package": "0402"}))
        .await;
        assert_eq!(response.status_code(), StatusCode::OK);

        let response = server.post("/jlc/v1")
        .json(&json!({"designator_prefix": "C", "mpn": "generic_inductor", "type": "inductor", "value": {"unit": "nanohenry", "min_val": 80.0, "max_val": 120.0, "nominal": 100.0}}))
        .await;
        assert_eq!(response.status_code(), StatusCode::OK);
    }
}
