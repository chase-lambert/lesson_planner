use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, PgPool};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<PgPool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub hashed_password: String,
}
