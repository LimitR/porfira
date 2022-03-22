extern crate argon2;
extern crate core;
extern crate dotenv;


use dotenv::dotenv;
use std::env;
use std::path::Path;

use actix_web::{body, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
mod ability;
mod plugins;
mod users;
mod db;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    //ability::ability_user::check_ability("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6IjI4ZWI2OTQyLWM5ODAtNDI3Mi1hMmEyLWYyY2Y5YWI0MDk0MSIsImxvZ2luIjoiYWRtaW4iLCJhYmlsaXR5IjozfQ.sbe__eYc_dLNueRQXGJ7ebCzOJ6eqGZdSa_dRNag-IQ".to_string());
    HttpServer::new(|| {
        App::new()
            .route("/create", web::post().to(users::routs::post_create_user))
            .route("/db", web::post().to(users::routs::create_data_base))
            .route("/add", web::post().to(users::routs::post_add_user))
            .route("/get/id", web::post().to(users::routs::get_get_user_i32))
            .route("/get/uuid", web::post().to(users::routs::get_get_user_uuid))
            .route("/all", web::post().to(users::routs::get_get_all_db))
            .route("/registration", web::post().to(users::routs::registration))
            .route("/create/post", web::post().to(users::routs::create_post))
            .route("/login", web::post().to(users::routs::login))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
