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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Person {
    pub name: String,
    pub data: Option<Vec<u8>>,
}

pub fn get_hello() -> String {
    "Hello".to_string()
}

pub async fn create_data_base(table_name: String) -> Result<u64, postgres::Error> {
    let mut conn = Client::connect("host=localhost user=postgres", NoTls).unwrap();
    conn.execute(
        "CREATE TABLE users (
           id              uuid DEFAULT uuid_generate_v4 (),
           name            VARCHAR NOT NULL,
           data            BYTEA
         )",
        &[&table_name],
    )
}

pub async fn add_user(user: web::Json<Person>) {
    let mut conn = Client::connect("host=localhost user=postgres", NoTls).unwrap();
    conn.execute(
        "INSERT INTO users (name, data) VALUES ($1, $2)",
        &[&user.name, &user.data],
    )
    .unwrap();
}

pub async fn get_user_num(id: i32) -> Result<Vec<Row>, postgres::Error> {
    let mut conn = Client::connect("host=localhost user=postgres", NoTls)
        .unwrap_or_else(|error| panic!("ОШИБКА ЕБАТЬ - {}", error));
    conn.query("SELECT * FROM person WHERE id = $1", &[&id])
}

pub async fn get_user_uuid(id: String) -> Result<Vec<Row>, postgres::Error> {
    let mut conn = Client::connect("host=localhost user=postgres", NoTls)
        .unwrap_or_else(|error| panic!("ОШИБКА ЕБАТЬ - {}", error));
    let _string = uuid::Uuid::parse_str(&id).unwrap();
    conn.query("SELECT * FROM users WHERE id = $1::uuid", &[&_string])
}

pub async fn get_all_db(db_name: String) -> Result<Vec<Row>, postgres::Error> {
    let mut sql = String::from("SELECT name FROM ");
    sql.push_str(&db_name.to_owned());
    let mut conn = Client::connect("host=localhost user=postgres", NoTls)
        .unwrap_or_else(|error| panic!("ОШИБКА ЕБАТЬ - {}", error));
    conn.query(&sql, &[])
}

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

pub async fn create_post(login: String, text: String, img: Vec<String>) {
    let mut conn = Client::connect(&dotenv::var("DB_CONN").unwrap() as &str, NoTls)
        .unwrap_or_else(|error| panic!("Error from connections a database"));
    conn.query(
        "CREATE TABLE IF NOT EXISTS comments (\
    id serial primary key,\
    )",
        &[],
    );
    conn.query(
        "INSERT INTO post(login, text, img) VALUES($1, $2)",
        &[&login, &text, &img],
    );
}
