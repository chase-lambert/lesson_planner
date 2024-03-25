use crate::types::{NewUser, User};
use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use sqlx::{types::Uuid, PgPool};

async fn create_user(pool: &PgPool, new_user: NewUser) -> Result<User> {
    let hashed_password = hash_password(&new_user.password)?;
    let user_id = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO users (id, username, first_name, last_name, email, hashed_password) VALUES ($1, $2, $3, $4, $5, $6)",
        user_id,
        new_user.username,
        new_user.first_name,
        new_user.last_name,
        new_user.email,
        hashed_password
    )
        .execute(pool)
        .await?;

    Ok(User {
        id: user_id,
        username: new_user.username,
        first_name: new_user.first_name,
        last_name: new_user.last_name,
        email: new_user.email,
        hashed_password,
    })
}

fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("Password hashing error: {e}"))?
        .to_string();

    Ok(password_hash)
}
