use crate::users::serviсe;

use crate::plugins::response;
use crate::users::schema::*;
use actix_web::{
    body, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use postgres::types::ToSql;
use postgres::{Client, NoTls, Row};
use serde::{Deserialize, Serialize};
use serde_json::*;
use std::error::Error;
use std::ops::{Add, Rem};
use std::sync::Mutex;

pub async fn registration(data: web::Json<UserRegistration>) -> impl Responder {
    let res = serviсe::post_registration(data.0).await;
    let mut response = response::Response::new();
    match res {
        Ok(res) => response.get_message(&res),
        Err(res) => {
            response.get_message("Error");
            response.get_error(true);
        }
    }
    HttpResponse::Ok().body(json!(response))
}

pub async fn login(data: web::Json<UserLogin>) -> impl Responder {
    let res = serviсe::post_login(data.0).await;
    let mut response = response::Response::new();
    match res {
        Ok(res) => response.get_message(&res),
        Err(res) => {
            response.get_error(true);
            response.get_message(&res);
        }
    };
    HttpResponse::Ok().body(json!(response))
}