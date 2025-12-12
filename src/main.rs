// mod db;

use axum::extract::{Path, State};
use axum::routing::delete;
use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use chrono::prelude::*;
use std::env;
pub mod errors;
mod models;
use models::{CreateEntry, Entry};
mod db;
use db::DB;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    tracing::info!("Starting server...");

    let database_url = env::var("DATABASE_URL").unwrap_or("sqlite:brag.db?mode=rwc".to_string());

    let db = DB::new(database_url)
        .await
        .expect("Cannot connect to the database.");

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(get_entries))
        // `POST /entries` goes to `create_entry`
        .route("/entries", post(create_entry))
        .route("/entries/{id}", delete(delete_entry))
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn get_entries(State(pool): State<SqlitePool>) -> (StatusCode, Json<Vec<Entry>>) {
    let result = sqlx::query_as("SELECT id, description, link, date FROM entries")
        .fetch_all(&pool)
        .await;
    match result {
        Ok(entries) => (StatusCode::OK, Json(entries)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new())),
    }
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
        .map_err(|e| {
            tracing::error!("Failed to create entry: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

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

async fn delete_entry(Path(id): Path<i64>, State(pool): State<SqlitePool>) -> StatusCode {
    let _ = sqlx::query("DELETE FROM entries WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete entry: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        });
    StatusCode::OK
}
