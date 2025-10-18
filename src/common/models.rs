use serde::{Serialize,Deserialize };

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUser {
    pub id: i64,
    pub name: String,
}