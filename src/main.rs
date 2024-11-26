use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_post`
        .route("/users", post(create_post));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    println!("Hello World!");
    "Hello, World!"
}

async fn create_post(
    // this argument tells axum to parse the request body
    // as JSON into a `CreatePost` type
    Json(payload): Json<CreatePost>,
) -> (StatusCode, Json<Post>) {
    // insert your application logic here
    let p = Post {
        id: 1337,
        text: payload.text,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(p))
}

// the input to our `create_post` handler
#[derive(Deserialize)]
struct CreatePost {
    text: String,
}

// the output to our `create_post` handler
#[derive(Serialize)]
struct Post {
    id: u64,
    text: String,
}