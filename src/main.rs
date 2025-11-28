mod db;

use std::collections::HashMap;
use std::env;
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
use sqlx::{FromRow, Sqlite, SqlitePool};

type Db = Arc<RwLock<HashMap<i64, Entry>>>;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    tracing::info!("Starting server...");

    let db = Db::default();
    let database_url = env::var("DATABASE_URL").unwrap_or("sqlite:brag.db?mode=rwc".to_string());
    let pool = SqlitePool::connect(&database_url).await.expect("Can't connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(get_entries))
        // `POST /entries` goes to `create_entry`
        .route("/entries", post(create_entry))
        //.route("/entries/{id}", delete(delete_entry) )
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn get_entries(State(pool): State<SqlitePool>) -> Result<Json<Vec<Entry>>,StatusCode> {
    //let m = db.read().unwrap();
    let result: Vec<Entry> = sqlx::query_as("SELECT id, description, link, date FROM entries")
        .fetch_all(&pool).await.map_err(|e| {
        tracing::error!("Failed to fetch entries: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(result))

}

async fn create_entry(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateEntry>,
) -> Result<Json<Entry>, StatusCode> {
    // insert your application logic here


    let date = Utc::now().timestamp();
    let result = sqlx::query("INSERT INTO entries (description, link, date) VALUES (?, ?, ?)")
        .bind(&payload.description)
        .bind(&payload.link)
        .bind(date)
        .execute(&pool)
        .await
        .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;

    let entry_id = result.last_insert_rowid();
    let entry = Entry {
        id: entry_id,
        description: payload.description,
        //collaborators: payload.collaborators,
        link: payload.link,
        date,
    };
    Ok(Json(entry))
    // this will be converted into a JSON response
    // with a status code of `201 Created`

}

// async fn delete_entry(Path(id): Path<i64>, State(pool): State<SqlitePool>) -> StatusCode {
//     let mut w = db.write().unwrap();
//     w.remove(&id);
//     info!("{w:?}");
//     StatusCode::FOUND
//
// }
#[derive(Deserialize, Serialize, Clone, Debug)]
struct Collaborator {
    id: i64,
    name: String,
    team: String
}

#[derive(Deserialize, Serialize, Clone, Debug, FromRow)]
struct Entry {
    id: i64,
    description: String,
    //collaborators: Vec<Collaborator>,
    link: Option<String>,
    date: i64,
}


// the input to our `create_entry` handler
#[derive(Deserialize)]
struct CreateEntry {
    description: String,
    //collaborators: Vec<Collaborator>,
    link: Option<String>,
}

