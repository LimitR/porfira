extern crate postgres;
use std::fmt::Error;
use actix_web::web;
use postgres::{Connection, SslMode, Column};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use self::postgres::Statement;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}

pub fn get_hello() -> String{
    "Hello".to_string()
}

pub async fn create_data_base(){
        let conn =
        Connection::connect(
            "postgresql://postgres@localhost",
            &SslMode::None)
        .unwrap();
    conn.execute(
        "CREATE TABLE person (
           id              SERIAL PRIMARY KEY,
           name            VARCHAR NOT NULL,
           data            BYTEA
         )",
        &[]).unwrap();
}

pub async fn add_user(user: web::Json<Person>) {
    let conn = Connection::connect("postgresql://postgres@localhost", &SslMode::None)
        .unwrap();

    conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
                 &[&user.name, &user.data]).unwrap();
}

pub async fn get_user(id: web::Json<i32>) -> String {
    // let conn =
    //     Connection::connect(
    //         "postgresql://postgres@localhost",
    //         &SslMode::None)
    //     .unwrap();
    //
    // let res = conn.prepare("SELECT name FROM person WHERE id = $1").unwrap();
    // res.columns()
    "Ok".to_string()
}