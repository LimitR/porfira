extern crate postgres;
use std::fmt::Error;
use actix_web::web;
use actix_web::web::Json;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use self::postgres::{Statement, NoTls, Client, Row};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub data: Option<Vec<u8>>
}

pub fn get_hello() -> String{
    "Hello".to_string()
}

pub async fn create_data_base(){
    let mut conn = Client::connect("host=localhost user=postgres", NoTls)
        .unwrap();
    conn.batch_execute(
        "CREATE TABLE person (
           id              SERIAL PRIMARY KEY,
           name            VARCHAR NOT NULL,
           data            BYTEA
         )").unwrap();
}

pub async fn add_user(user: web::Json<Person>) {
    let mut conn = Client::connect("host=localhost user=postgres", NoTls)
        .unwrap();

    conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
                 &[&user.name, &user.data]).unwrap();
}

pub async fn get_user(id: i32) -> Result<Vec<Row>, postgres::Error> {
    let mut conn = Client::connect("host=localhost user=postgres", NoTls)
        .unwrap();
    conn.query("SELECT name FROM person WHERE id = $1", &[&id])
}