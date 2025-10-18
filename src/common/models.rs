use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Separate entity as in the future maybe we want to add a created_at field
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
}

// Separate entity as in the future maybe we want to also store a password
#[derive(Debug, Clone, Deserialize)]
pub struct NewUser {
    pub id: i64,
    pub name: String,
}
