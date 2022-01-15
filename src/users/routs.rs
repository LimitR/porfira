use crate::users::serviсe;
use actix_web::{
    body, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use postgres::types::ToSql;
use postgres::{Client, NoTls, Row};
use serde::{Deserialize, Serialize};
use serde_json::*;
use std::error::Error;
use std::ops::{Add, Rem};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SomeData {
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPub {
    id: i16,
    login: String,
    first_name: String,
    last_name: String,
    link_from_db_id: i16,
}

pub async fn post_create_user(user: web::Json<UserPub>) -> impl Responder {
    println!("{:?}", user);
    HttpResponse::Ok().body(serviсe::get_hello())
}

pub async fn create_data_base(table_name: web::Json<SomeData>) -> impl Responder {
    let mut result = serviсe::create_data_base(table_name.0.value.to_owned().clone()).await;
    match result {
        Ok(t) => HttpResponse::Ok().body("Data base created"),
        Err(error) => HttpResponse::Ok().body(error.to_string()),
    }
}

pub async fn post_add_user(user: web::Json<serviсe::Person>) -> impl Responder {
    serviсe::add_user(user).await;
    HttpResponse::Ok().body("User added")
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IdUserI32 {
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IdUserString {
    pub id: String,
}

pub async fn get_get_user_i32(id: web::Json<IdUserI32>) -> impl Responder {
    let val = id.0.id.to_owned().clone();
    let mut res = serviсe::get_user_num(val)
        .await
        .unwrap_or_else(|error| panic!("ОШИБКА ЕБАТЬ - {}", error));

    let value: &str = res[0].get(0);
    HttpResponse::Ok().body(value.to_owned())
}

pub async fn get_get_user_uuid(id: web::Json<IdUserString>) -> impl Responder {
    let val = id.0.id.to_owned().clone();
    let mut res = serviсe::get_user_uuid(val)
        .await
        .unwrap_or_else(|error| panic!("ОШИБКА ЕБАТЬ - {}", error));

    let value: &str = res[0].get(0);
    HttpResponse::Ok().body(value.to_owned())
}
