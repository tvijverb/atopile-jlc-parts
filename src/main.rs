

use axum::{Extension, Router};
use sqlx::postgres::PgPoolOptions;


use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use utoipauto::utoipauto;
use clap::Parser;
use dotenv::dotenv;

pub mod jlc;


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
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
}

#[tokio::main]
async fn main() {
    // load .env file
    dotenv().ok();

    // initialize tracing
    let _filter = "atopile_jlc_parts=info,sqlx=warn";

    let args = Args::parse();

    // initialize tracing
    tracing_subscriber::registry()
        .with(fmt::layer())
        // .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::new(_filter))
        .init();

    let app_state = AppState {
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
        .layer(Extension(pool_extension));

        // // Leave this commented out for now, halving the performance of the server
        // .layer(
        //     TraceLayer::new_for_http()
        //         .make_span_with(|_request: &Request<_>| info_span!("http_request"))
        //         .on_request(|_request: &Request<_>, _span: &Span| {
        //             tracing::info!("request received: {}", _request.uri().path());
        //         })
        //         .on_response(
        //             |_response: &Response<_>, _latency: Duration, _span: &Span| {
        //                 let status = _response.status().as_u16();
        //                 if status >= 500 {
        //                     tracing::error!("response sent: {}", _response.status().as_u16());
        //                 } else {
        //                     tracing::info!(
        //                         "response sent: {}. with latency: {}",
        //                         _response.status().as_u16(),
        //                         _latency.as_millis()
        //                     );
        //                 }
        //             },
        //         ),
        // );

    // run our app with hyper, listening globally on port 3000
    tracing::info!("Started Axum server on port 3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
