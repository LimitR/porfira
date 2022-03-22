extern crate postgres;

use self::postgres::types::private::BytesMut;
use self::postgres::types::{IsNull, Type};
use self::postgres::{Client, NoTls, Row, Statement};
use crate::users::auth::auth;
use crate::users::schema::*;
use actix_web::web;
use actix_web::web::Json;
use jsonwebtoken::{
    decode, decode_header, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use postgres_types::ToSql;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::any::Any;
use std::fmt::Error;
use std::str::FromStr;
use std::sync::Mutex;
use uuid::Uuid;

pub async fn post_registration(data: UserRegistration) -> Result<String, postgres::Error> {
    let mut conn = Client::connect(&dotenv::var("DB_CONN").unwrap() as &str, NoTls).unwrap();
    let jwt_data = auth::create_JWT(&data.clone().for_jwt());
    let hash_password = auth::hash_password(&data.password);
    let uuid = conn.query(
        "INSERT INTO users(password_hash, login, ability, email) VALUES($1, $2, $3, $4)",
        &[&hash_password, &data.login, &data.ability, &data.email],
    );
    match uuid {
        Ok(res) => Ok(jwt_data),
        Err(res) => Err(res),
    }
}

pub async fn post_login(data: UserLogin) -> Result<String, String> {
    let match_password = auth::check_password(&data.login, &data.password);
    let mut jwt = String::new();
    let res = if match_password.0 {
        jwt = auth::create_JWT(UserJWTSchema {
            id: match_password.2.to_string(),
            login: data.login,
            ability: match_password.1,
        });
        auth::save_jwt_token(match_password.2.to_string(), jwt.clone());
        Ok(jwt)
    } else {
        Err("Неверный логин или пароль".to_string())
    };
    res
}