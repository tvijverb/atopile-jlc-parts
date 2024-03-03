use std::time::Duration;

use axum::{http::{Request, Response}, Extension, Router};
use polars::prelude::*;
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;
use tracing::{info_span, Span};
use tracing_subscriber::prelude::*;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use utoipauto::utoipauto;
use clap::Parser;
use dotenv::dotenv;

pub mod jlc;


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// DB_URI
    #[arg(short, long, env)]
    database_url: String,
}

#[utoipauto]
#[derive(OpenApi)]
#[openapi(info(title = "JLCPCB Part Selector API", version = "1.0.0"))]
pub struct ApiDoc;

#[derive(Clone)]
pub struct AppState {
    polars_df: LazyFrame,
    resistor_df: LazyFrame,
    capacitor_df: LazyFrame,
    inductor_df: LazyFrame,
}

#[tokio::main]
async fn main() {
    // load .env file
    dotenv().ok();

    // initialize tracing
    let _filter = "atopile-jlc-parts=debug";

    let args = Args::parse();

    // initialize tracing
    tracing_subscriber::registry()
        // .with(fmt::layer())
        // .with(EnvFilter::new(filter))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app_state = AppState {
        polars_df: LazyFrame::scan_parquet("components.parquet", ScanArgsParquet::default())
            .unwrap(),
        resistor_df: LazyFrame::scan_parquet("resistors.parquet", ScanArgsParquet::default())
            .unwrap(),
        capacitor_df: LazyFrame::scan_parquet("capacitors.parquet", ScanArgsParquet::default())
            .unwrap(),
        inductor_df: LazyFrame::scan_parquet("inductors.parquet", ScanArgsParquet::default())
            .unwrap(),
    };

    // set up connection pool
    let pool_extension = PgPoolOptions::new()
        .max_connections(40)
        .connect(args.database_url.as_str())
        .await
        .expect("unable to open db connection");

    // build our application with a route
    let app = Router::new()
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()))
        .with_state(app_state.clone())
        .nest("/jlc", jlc::router().with_state(app_state))
        .layer(Extension(pool_extension))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|_request: &Request<_>| info_span!("http_request"))
                .on_request(|_request: &Request<_>, _span: &Span| {
                    tracing::info!("request received: {}", _request.uri().path());
                })
                .on_response(
                    |_response: &Response<_>, _latency: Duration, _span: &Span| {
                        let status = _response.status().as_u16();
                        if status >= 500 {
                            tracing::error!("response sent: {}", _response.status().as_u16());
                        } else {
                            tracing::info!(
                                "response sent: {}. with latency: {}",
                                _response.status().as_u16(),
                                _latency.as_millis()
                            );
                        }
                    },
                ),
        );

    // run our app with hyper, listening globally on port 3000
    tracing::info!("Started Axum server on port 3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
