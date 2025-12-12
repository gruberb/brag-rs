use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, Clone, Debug, FromRow)]
pub struct Entry {
    pub id: i64,
    pub description: String,
    //collaborators: Vec<Collaborator>,
    pub link: Option<String>,
    pub date: i64,
}

// the input to our `create_entry` handler
#[derive(Deserialize)]
pub struct CreateEntry {
    pub description: String,
    //collaborators: Vec<Collaborator>,
    pub link: Option<String>,
}
