use anyhow::{Context, Result};
use sqlx::PgPool;

use crate::db::{create_user, get_user_by_email};
use crate::types::NewUser;

pub async fn create_dummy_user(pool: &PgPool) -> Result<()> {
    let dummy_user = NewUser {
        username: "johndoe".to_string(),
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "john@example.com".to_string(),
        password: "password".to_string(),
    };

    let new_user = create_user(&pool, dummy_user)
        .await
        .context("couldn't create dummy user")?;

    tracing::debug!("New user created: {new_user:?}");
    println!("New user created: {new_user:?}");

    Ok(())
}

pub async fn retrieve_dummy_user_with_email(pool: PgPool) -> Result<()> {
    let email = "john@example.com".to_string();

    let user = get_user_by_email(pool, &email)
        .await
        .context("Couldn't get user")?;

    tracing::debug!("User retrieved: {user:?}");

    Ok(())
}
