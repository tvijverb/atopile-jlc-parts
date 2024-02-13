use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
    extract::State
};
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use utoipauto::utoipauto;
use utoipa::{IntoParams, ToSchema};

pub mod jlc_models;
pub mod jlc_router;
pub mod jlc_part_finder;
pub mod jlc_searchers;


#[utoipauto]
#[derive(OpenApi)]
#[openapi(info(title = "JLCPCB Part Selector API", version = "1.0.0"))]
pub struct ApiDoc;

struct AppState {
    user_string: Mutex<String>
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let app_state = Arc::new(AppState { user_string: Mutex::new("Nom".to_string()) });

    // build our application with a route
    let app = Router::new()
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()))
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .with_state(app_state)
        .nest("/jlc", jlc_router::router());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// EXAMPLES

// basic handler that responds with a static string
#[utoipa::path(get, path = "/",
responses(
    (status = 200, description = "Hello world") // body = [Todo])
))]

async fn root(State(state): State<Arc<AppState>>,) -> String {
    state.user_string.lock().unwrap().clone()
}

/// Create a new user
#[utoipa::path(post, path = "/users",
request_body = CreateUser,
responses(
    (status = 201, description = "User created", body = [User])
)
)]
async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize, IntoParams, ToSchema)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize, ToSchema)]
struct User {
    id: u64,
    username: String,
}