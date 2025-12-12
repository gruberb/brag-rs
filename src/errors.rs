use sqlx::Error as SQLXError;
use sqlx::migrate::MigrateError;

#[derive(Debug)]
pub enum BragError {
    DBError(SQLXError),
    MigrationError(MigrateError),
}
