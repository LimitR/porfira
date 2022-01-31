extern crate postgres;

use std::any::Any;
use std::str::FromStr;
use self::postgres::types::private::BytesMut;
use self::postgres::types::{IsNull, Type};
use self::postgres::{Client, NoTls, Row, Statement};
use crate::users::routs::{SomeData};
use actix_web::web;
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Error;
use uuid::Uuid;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey, decode_header};

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


pub async fn post_registration(login: String, password: String ) -> Result<Vec<Row>, postgres::Error>{
    let mut conn = Client::connect("host=localhost user=postgres", NoTls)
        .unwrap_or_else(|error| panic!("ОШИБКА ЕБАТЬ - {}", error));
    let uuid_password = encode(&Header::default(), &password, &EncodingKey::from_secret("secret".as_ref())).unwrap();
    let ttt = decode_header(&uuid_password).unwrap();
    println!("{:#?}", ttt);
    conn.query("INSERT INTO registration(password, login) VALUES($1, $2)", &[&uuid_password, &login])
}