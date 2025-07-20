use axum::{
    routing::{get, post},
    http::StatusCode,
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    tracing::info!("Started logging");

     // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Root"
}

async fn create_user(
    Json(payload): Json<CreateUserDTO>
) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1,
        username: payload.username.clone(),
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUserDTO {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

