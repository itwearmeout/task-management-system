use argon2::{Argon2, PasswordHash, password_hash::{SaltString, rand_core::OsRng}};
use axum::{Json, Extension, http::StatusCode};
use crate::{error::{Error, Result}, ApiContext, user::auth::{generate_token, AuthUserClaim}};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserBody<T>{
    user: T,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct User{
    username: String,
    email : Option<String>,
    angkatan :i32,
    token :String,
}

#[derive(serde::Deserialize)]
pub struct CreateUser{
    username: String,
    email : Option<String>,
    password :String,
    angkatan :i32,
}

#[derive(serde::Deserialize)]
pub struct LoginUser{
    username: String,
    password: String,
}

#[derive(serde::Deserialize)]
pub struct DeleteUser {}

pub async fn user_login(Extension(ctx): Extension<ApiContext>, Json(req): Json<UserBody<LoginUser>>)->Result<Json<UserBody<User>>>{
    let user = sqlx::query!(
        r#"
            select user_id, email, username, password_hash, angkatan
            from users
            where username = $1
        "#,
        req.user.username,
    )
    .fetch_optional(&ctx.db)
    .await?
    .ok_or(Error::UnprocessableEntity("Username not found".to_string()))?;

    password_login(req.user.password, user.password_hash).await?;

    let token = generate_token(&user.user_id, &ctx.config.hmac_key)?;

    Ok(Json(UserBody { user: User 
        { username: user.username, 
            email: user.email, 
            angkatan: user.angkatan,
            token, 
        } 
    }))
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
                        return Error::UnprocessableEntity("Username is taken".to_string());
                    }
                    "user_email_key" => {
                        return Error::UnprocessableEntity("Email is taken".to_string());
                    }
                    _ => {}
                }
            }
        }
        Error::Sqlx(e)
    })?;
    let token = generate_token(&user_id, &ctx.config.hmac_key)?;
    Ok(Json(UserBody { user: User 
        { 
            username: req.user.username, 
            email: req.user.email, 
            angkatan: req.user.angkatan,
            token, 
        } 
    }))
}

async fn password_hasher(password: String)->Result<String>{
    tokio::task::spawn_blocking(move || -> Result<String>{
        let salt = SaltString::generate(&mut OsRng);

        let hash = PasswordHash::generate
            (Argon2::default(), password, &salt)
            .map_err(|_| Error::HashError)?;

        Ok(hash.to_string())
    })
    .await
    .map_err(|_| Error::HashError)?
}

async fn password_login(password: String, password_hash: String)-> Result<()> {
        tokio::task::spawn_blocking(move || -> Result<()> {
            let hash = PasswordHash::new(&password_hash)
                .map_err(|_| Error::HashError)?;
            hash.verify_password(&[&Argon2::default()], password)
                .map_err(|_| Error::InvalidPassword)?;
            Ok(())
        })
        .await
        .map_err(|_| Error::HashError)??;

        Ok(())
}

pub async fn user_delete(
    Extension(ctx): Extension<ApiContext>,
    claims: AuthUserClaim,
) -> Result<StatusCode> {
    sqlx::query!(
        r#"
            delete from users
            where user_id = $1
        "#,
        claims.user_id,
    )
    .execute(&ctx.db)
    .await
    .map_err(Error::Sqlx)?;

    Ok(StatusCode::NO_CONTENT)
}