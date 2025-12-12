// use chrono::prelude::*;
use super::errors::BragError;
use sqlx::SqlitePool;

pub struct DB {
    connections: SqlitePool,
    database_url: String,
}

impl DB {
    pub async fn new(url: String) -> Result<Self, BragError> {
        let pool = SqlitePool::connect(&url)
            .await
            .map_err(|e| BragError::DBError(e))?;

        Ok(Self {
            connections: pool,
            database_url: url,
        })
    }

    pub async fn run_migrations(&self) -> Result<(), BragError> {
        sqlx::migrate!("./migrations")
            .run(&self.connections)
            .await
            .map_err(|e| BragError::MigrationError(e))
    }
}
