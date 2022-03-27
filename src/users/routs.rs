use std::pin::Pin;
use std::task::{Context, Poll};
use crate::users::serviсe;
use crate::models::user::*;
use crate::plugins::response;
use actix_web::{body, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result, HttpResponseBuilder};
use actix_web::body::MessageBody;
use actix_web::web::{Bytes, Json};
use serde::{Deserialize, Serialize};
use serde_json::*;
use sqlx::PgPool;
use crate::body::BodySize;


pub async fn registration(pool: web::Data<PgPool>, data: web::Json<NewUser>) -> impl Responder  {
    let res = serviсe::post_registration(pool,data.0).await;
    let mut response = response::Response::new();
    match res {
        Ok(res) => response.get_message(&res),
        Err(res) => {
            response.get_message("Error");
            response.get_error(true);
        }
    };
    HttpResponse::Ok().body(response.message)
}

pub async fn login(pool: web::Data<PgPool>, data: web::Json<NewUser>) -> impl Responder   {
    let res = serviсe::post_login(pool,data.0).await;
    let mut response = response::Response::new();
    match res {
        Ok(res) => response.get_message(&res),
        Err(res) => {
            response.get_error(true);
            response.get_message(&res);
        }
    };
    HttpResponse::Ok().body(response.message)
}