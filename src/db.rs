use crate::types::{NewUser, User};
use anyhow::{Context, Result};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use sqlx::{types::Uuid, PgPool};

fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("Password hashing error: {e}"))?
        .to_string();

    Ok(password_hash)
}

pub async fn create_user(pool: &PgPool, new_user: NewUser) -> Result<User> {
    let hashed_password = hash_password(&new_user.password).context("Could not hash password")?;
    let user_id = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO users (id, first_name, last_name, email, hashed_password) VALUES ($1, $2, $3, $4, $5)",
        user_id,
        new_user.first_name,
        new_user.last_name,
        new_user.email,
        hashed_password
    )
        .execute(pool)
        .await
        .context("Failed to insert user to db")?;

    Ok(User {
        id: user_id,
        first_name: new_user.first_name,
        last_name: new_user.last_name,
        email: new_user.email,
        hashed_password,
    })
}

pub async fn delete_user(pool: &PgPool, user_id: Uuid) -> Result<()> {
    sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<User> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
        .fetch_one(pool)
        .await
        .context("Could not find user")?;

    Ok(user)
}

pub fn verify_password(plain: &str, hashed: &str) -> bool {
    match PasswordHash::new(hashed) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(plain.as_bytes(), &parsed_hash)
            .is_ok(),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password = "password";
        let password_hash = hash_password(password).expect("Failed to hash password");
        let parsed_hash =
            PasswordHash::new(&password_hash).expect("Failed to parse hashed password");

        assert!(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok());
    }
}
