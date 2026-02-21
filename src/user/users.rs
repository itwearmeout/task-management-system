use anyhow::{Context};
use argon2::{Argon2, PasswordHash, PasswordHasher, password_hash::{rand_core::OsRng, SaltString}};
use axum::{Json, Extension};
use clap::{builder::Str, error};
use crate::{error::{Error, Result}, user::ApiContext};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserBody<T>{
    user: T,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct User{
    username: String,
    email : String,
    angkatan :i32,
}

#[derive(serde::Deserialize)]
pub struct CreateUser{
    username: String,
    email : String,
    password :String,
    angkatan :i32,
}

#[derive(serde::Deserialize)]
pub struct LoginUser{
    username: String,
    password: String,
}

pub async fn user_login(){

}

pub async fn user_create(Extension(ctx): Extension<ApiContext>, Json(req): Json<UserBody<CreateUser>>)->Result<Json<UserBody<User>>>{
    let hash_password = password_hasher(req.user.password).await?;

    let user_id = sqlx::query_scalar!(
        r#"insert into "users" (username, password_hash, email, angkatan) values($1, $2, $3, $4) returning user_id"#,
        req.user.username,
        hash_password,
        req.user.email,
        req.user.angkatan as i32
    )
    .fetch_one(&ctx.db)
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(db_err) = &e {
            if let Some(constraint) = db_err.constraint() {
                match constraint {
                    "user_username_key" => {
                        return Error::Forbidden;
                    }
                    "user_email_key" => {
                        return Error::Forbidden;
                    }
                    _ => {}
                }
            }
        }
        Error::Sqlx(e)
    })?;
    Ok(Json(UserBody { user: User { username: req.user.username, email: req.user.email, angkatan: req.user.angkatan } }))
}

async fn password_hasher(password: String)->Result<String>{
    Ok(tokio::task::spawn_blocking(move || -> anyhow::Result<String>{
        let salt = SaltString::generate(&mut OsRng);

        let hash = PasswordHash::generate
            (Argon2::default(), password, &salt)
            .map_err(|e| anyhow::anyhow!("Failed to hash password: {e}"))?.to_string();

        Ok(hash)
    }).await.context("Hash error")??)
}