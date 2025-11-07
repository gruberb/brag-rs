use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use axum::extract::{Path, State};
use axum::routing::delete;
use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use tracing::info;

type Db = Arc<RwLock<HashMap<i64, Entry>>>;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    tracing::info!("Starting server...");

    let db = Db::default();
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(get_entries))
        // `POST /entries` goes to `create_entry`
        .route("/entries", post(create_entry))
        .route("/entries/{id}", delete(delete_entry) )
        .with_state(db);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn get_entries(State(db): State<Db>) -> (StatusCode, Json<Vec<Entry>>){
    let m = db.read().unwrap();
    let result = m.values().cloned().collect();
    (StatusCode::OK, Json(result))

}

async fn create_entry(
    State(db): State<Db>,
    Json(payload): Json<CreateEntry>,
) -> (StatusCode, Json<Entry>) {
    // insert your application logic here
    let entry = Entry {
        description: payload.description,
        collaborators: payload.collaborators,
        link: payload.link,
       ..Default::default()
    };

    let mut w = db.write().unwrap();
    w.insert(entry.date, entry.clone());

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(entry))
}

async fn delete_entry(Path(id): Path<i64>, State(db): State<Db>) -> StatusCode {
    let mut w = db.write().unwrap();
    w.remove(&id);
    info!("{w:?}");
    StatusCode::FOUND

}
#[derive(Deserialize, Serialize, Clone, Debug)]
struct Collaborator {
    name: String,
    team: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
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

// the input to our `create_entry` handler
#[derive(Deserialize)]
struct CreateEntry {
    description: String,
    collaborators: Vec<Collaborator>,
    link: Option<String>,
}

