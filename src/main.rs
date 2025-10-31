use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use chrono::prelude::*;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    tracing::info!("Starting server...");

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/entries", post(create_entry));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_entry(
    Json(payload): Json<CreateEntry>,
) -> (StatusCode, Json<Entry>) {
    // insert your application logic here
    let entry = Entry {
        description: payload.description,
        collaborators: payload.collaborators,
        link: payload.link,
       ..Default::default()
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(entry))
}

#[derive(Deserialize, Serialize)]
struct Collaborator {
    name: String,
    team: String
}

#[derive(Deserialize, Serialize)]
struct Entry {
    description: String,
    collaborators: Vec<Collaborator>,
    link: Option<String>,
    date: i64,
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            description: "".to_string(),
            collaborators: vec![],
            link: None,
            date: Utc::now().timestamp()
        }
    }
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateEntry {
    description: String,
    collaborators: Vec<Collaborator>,
    link: Option<String>,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: 0,
            username: "".to_string(),
        }
    }
}
