use crate::models::user::*;
use crate::plugins::response;
use crate::users::serviсe;
use actix_web;
use actix_web::web::{Bytes, Json};
use actix_web::{
     get, post, web, App, HttpRequest, HttpResponse, HttpResponseBuilder, HttpServer,
    Responder, Result,
};
use serde::{Deserialize, Serialize};
use serde_json::*;
use sqlx::PgPool;
use std::task::{Context, Poll};


pub async fn registration(pool: web::Data<PgPool>, data: web::Json<NewUser>) -> impl Responder {
    let result = serviсe::post_registration(pool, data.0).await;
    let mut response = response::Response::new();
    match result {
        Ok(res) => response.get_message(&res),
        Err(res) => {
            response.get_message("Error");
            response.get_error(true);
        }
    };
    HttpResponse::Ok().json(response)
}

pub async fn login(pool: web::Data<PgPool>, data: web::Json<UserLogin>) -> impl Responder {
    let res = serviсe::post_login(pool, data.0).await;
    let mut response = response::Response::new();
    match res {
        Ok(res) => response.get_message(&res),
        Err(res) => {
            response.get_error(true);
            response.get_message(&res);
        }
    };
    HttpResponse::Ok().json(response)
}
