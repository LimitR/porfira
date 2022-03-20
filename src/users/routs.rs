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

pub async fn get_get_all_db(db: web::Json<Some_json>) -> impl Responder {
    let mut res = serviсe::get_all_db(db.0.collection.unwrap()).await;
    let mut vec_str: Vec<String> = Vec::new();
    for element in res.unwrap().iter() {
        vec_str.push(element.clone().to_owned().get(0));
    }
    HttpResponse::Ok().body(json!({ "data": vec_str }))
}

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

pub async fn create_post(data: web::Json<PostText>) -> impl Responder {
    let res = serviсe::create_post(data.0.login, data.0.text, data.0.img).await;
    HttpResponse::Ok().body(json!({"data": "ok"}))
}
