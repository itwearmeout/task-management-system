use axum::Json;
use clap::builder::Str;
use crate::error::{Error, Result};

#[derive(serde::Serialize, serde::Deserialize)]
struct User{
    id :String,
    username: String,
    email :String,
    angkatan :u16,
}

#[derive(serde::Deserialize)]
pub struct CreateUser{
    id :String,
    username: String,
    email :String,
    angkatan :u16,
}


pub async fn user_login(){

}

pub async fn user_create(){

}