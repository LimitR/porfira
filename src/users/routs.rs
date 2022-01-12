use std::error::Error;
use std::ops::{Add, Rem};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, body, HttpRequest, Result};
use postgres::{Client, NoTls, Row};
use serde::{Deserialize, Serialize};
use crate::users::serviсe;
use serde_json::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPub {
    id: i16,
    login: String,
    first_name: String,
    last_name: String,
    link_from_db_id: i16
}

pub async fn post_create_user(user: web::Json<UserPub>) -> impl Responder{
    println!("{:?}", user);
    HttpResponse::Ok().body(serviсe::get_hello())
}


pub async fn create_data_base() -> impl Responder{
        serviсe::create_data_base();
    HttpResponse::Ok().body("Data base created")
}

pub async fn post_add_user(user: web::Json<serviсe::Person>) -> impl Responder{
        serviсe::add_user(user).await;
    HttpResponse::Ok().body("User added")
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct id_user{
    pub id: i32
}

pub async fn get_get_user(id: web::Json<id_user>) -> impl Responder{
    let val = &id.0.id.to_owned();
    let mut res = serviсe::get_user(val.to_owned().clone()).await.unwrap_or_else(|eroror|{
        panic!("ОШИБКА ЕБАТЬ - {}", eroror)
    });
    let value: &str = res[0].get(0);
    HttpResponse::Ok().body(value.to_owned())
}
