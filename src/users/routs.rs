extern crate postgres;
use std::error::Error;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, body, HttpRequest, Result};
use postgres::{Connection, SslMode};
use serde::{Deserialize, Serialize};
use crate::users::serviсe;

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

pub async fn get_get_user(id: web::Json<i32>)-> impl Responder{
    HttpResponse::Ok().body(serviсe::get_user(id).await)
}
