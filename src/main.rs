use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};

mod database;
mod handlers;
use database::create_database_if_inexistant;
use database::insert_post_in_database;
use handlers::Post;
use handlers::CreatePost;

#[tokio::main]
async fn main() {
    create_database_if_inexistant();
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /posts` goes to `create_post`
        .route("/posts", post(create_post));

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
    let p = insert_post_in_database(payload).unwrap();

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(p))
}
