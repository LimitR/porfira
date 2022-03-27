use argon2::{self, Config};
use dotenv::dotenv;
use jsonwebtoken::{
    decode, decode_header, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation
};
use serde::de::Unexpected::Str;
use serde::Serialize;
use std::env;
use actix_web::web;
use sqlx::{FromRow, PgPool, pool};
use sqlx::postgres::*;
use uuid::Uuid;
use crate::models::user::User;
use std::sync::Arc;

#[derive(sqlx::FromRow, Serialize, Debug, Clone)]
struct CheckUser {
    password_hash: String,
    ability: i32,
    id: String
}


pub fn create_JWT<T: Serialize>(data: T) -> String {
    encode(
        &Header::default(),
        &data, // Data for JWT
        &EncodingKey::from_secret(dotenv::var("SECRET").unwrap().as_ref()),
    )
    .unwrap()
}

pub fn hash_password(password: &str) -> String {
    let config = Config::default();
    argon2::hash_encoded(
        password.as_ref(),
        dotenv::var("SALT").unwrap().as_ref(),
        &config,
    )
    .unwrap()
}

pub async fn check_password<'a>(pool: web::Data<PgPool>, login: &'a str, password: &'a str) -> (bool, i32, String) {
    let mut ability: i32 = 0;
    let mut password_hash: String = "".to_string();
    let mut id = String::new();
    let user = sqlx::query_as::<_, CheckUser>(
            "SELECT password_hash, ability, id FROM users2 WHERE login = $1",)
        .bind(&login)
        .fetch_one(&*pool.as_ref())
        .await.unwrap();
    {
        password_hash = user.password_hash;
        ability = user.ability;
        id = user.id;
    }
    let res = argon2::verify_encoded(&password_hash, password.as_ref()).unwrap();
    (res, ability, id)
}

pub async fn save_jwt_token(pool: web::Data<PgPool>, uuid: String, token: String) {
    sqlx::query(
        "INSERT INTO tokens(ref_id, token) VALUES($1, $2)"
    )
        .bind::<String>(uuid)
        .bind::<String>(token)
        .execute(&*pool.as_ref());
}

pub async fn refresh_token(pool: web::Data<PgPool>, uuid: String, token: String) {
    sqlx::query(
        "UPDATE tokens SET token = $1 WHERE ref_id = $2"
    )
        .bind(&uuid)
        .bind(&token)
        .execute(&*pool.as_ref())
        .await;
}
