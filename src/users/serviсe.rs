use crate::users::auth::auth;
use crate::models::user::*;
use actix_web::web;
use actix_web::web::Json;
use jsonwebtoken::{
    decode, decode_header, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::any::Any;
use std::fmt::Error;
use std::str::FromStr;
use std::sync::Mutex;
use sqlx::{PgPool, postgres};
use uuid::Uuid;
use std::time::{Duration, SystemTime};
use argon2::Config;
use sqlx::postgres::*;
use crate::users::auth::auth::hash_password;


impl NewUser {
    pub fn create_user(self)-> User {
        User {
            login: self.login,
            password_hash: hash_password(&self.password),
            ability: 0,
            email: self.email,
            image: "http://assets.stickpng.com/images/585e4bcdcb11b227491c3396.png".to_string(),
            approved: false
        }
    }
    fn hash_password(password: &str) -> String {
        let config = Config::default();
        argon2::hash_encoded(
            password.as_ref(),
            dotenv::var("SALT").unwrap().as_ref(),
            &config,
        )
            .unwrap()
    }
    
    pub fn for_jwt(self, ability: i32) -> UserJWT {
        UserJWT {
            login: self.login,
            ability: ability,
        }
    }
}

pub async fn post_registration(pool: web::Data<PgPool>, data: NewUser) -> Result<String, sqlx::error::Error> {
    let jwt_data = auth::create_JWT(&data.clone().for_jwt(0));
    let user = data.create_user();
    let uuid = sqlx::query_as::<_, User>(
        "INSERT INTO users2(password_hash, login, ability, email, image, approved) VALUES($1, $2, $3, $4, $5, $6)")
        .bind(&user.password_hash)
        .bind(&user.login)
        .bind(&user.ability)
        .bind(&user.email)
        .bind(&user.image)
        .bind(&user.approved)
        .fetch_one(&**pool).await;
    match uuid {
        Ok(res) => Ok(jwt_data),
        Err(res) => Err(res),
    }
}

pub async fn post_login(pool: web::Data<PgPool>, data: NewUser) -> Result<String, String> {
    let match_password = auth::check_password(pool.clone(), &data.login, &data.password).await;
    let mut jwt = String::new();
    let res = if match_password.0 {
        jwt = auth::create_JWT(&data);
        auth::save_jwt_token(pool.clone(), match_password.2.to_string(), jwt.clone());
        Ok(jwt)
    } else {
        Err("Неверный логин или пароль".to_string())
    };
    res
}