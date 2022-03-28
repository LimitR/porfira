use crate::models::user::{User, UserCheckEmail};
use actix_web::web;
use argon2::{self, Config};
use dotenv::dotenv;
use jsonwebtoken::{
    decode, decode_header, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use sqlx::postgres::*;
use sqlx::{pool, FromRow, PgPool};
use std::env;
use uuid::Uuid;
use reqwest;

#[derive(sqlx::FromRow, Serialize, Debug, Clone)]
struct CheckUser {
    password_hash: String,
    ability: String,
    id: Uuid,
}

#[derive(Deserialize, Serialize, Debug)]
struct SuccessResponse {
    pub err: bool,
    pub email: String,
    pub message: String
}

#[derive(Deserialize, Serialize, Debug)]
struct ErrorResponse {
    pub err: bool,
    pub message: String
}

#[derive(Debug)]
enum ListResponseForSendMail {
    SuccessResponse,
    ErrorResponse
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

pub async fn check_password<'a>(
    pool: web::Data<PgPool>,
    login: &'a str,
    password: &'a str,
) -> (bool, i32, String) {
    let mut ability: i32 = 0;
    let mut password_hash: String = "".to_string();
    let mut id = String::new();
    let user = sqlx::query_as::<_, CheckUser>(
        "SELECT password_hash, ability, id FROM users2 WHERE login = $1",
    )
    .bind(&login.to_string())
    .fetch_one(&**pool)
    .await
    .unwrap();
    {
        password_hash = user.password_hash;
        ability = user.ability.parse::<i32>().unwrap();
        id = user.id.to_string();
    }
    let res = argon2::verify_encoded(&password_hash, password.as_ref()).unwrap();
    (res, ability, id)
}

pub async fn save_jwt_token(pool: web::Data<PgPool>, uuid: String, token: String) {
    sqlx::query("INSERT INTO tokens(ref_id, token) VALUES($1, $2)")
        .bind::<String>(uuid)
        .bind::<String>(token)
        .execute(&**pool);
}

pub async fn refresh_token(pool: web::Data<PgPool>, uuid: String, token: String) {
    sqlx::query("UPDATE tokens SET token = $1 WHERE ref_id = $2")
        .bind(uuid)
        .bind(token)
        .execute(&**pool)
        .await;
}

pub async fn send_checker_email(data: String) -> bool {
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:3006/")
        .json(&json!(
	{"token": data}
    ))
        .send()
        .await.unwrap().json::<SuccessResponse>().await.unwrap();
    res.err
}
