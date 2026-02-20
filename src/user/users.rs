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
    id :String,
    username: String,
    email : Option<String>,
    angkatan :u16,
}

#[derive(serde::Deserialize)]
pub struct CreateUser{
    username: String,
    email : Option<String>,
    password :String,
    angkatan :u16,
}

#[derive(serde::Deserialize)]
pub struct LoginUser{
    username: String,
    password: String,
}

pub async fn user_login(){

}

pub async fn user_create(Json(req): Json<UserBody<CreateUser>>, Extension(ctx): Extension<ApiContext>)->Result<Json<UserBody<User>>>{
    let password_hash = password_hasher(req.user.password).await?;
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