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
    HttpServer::new(|| {
        App::new()
            .route("/registration", web::post().to(users::routs::registration))
            .route("/login", web::post().to(users::routs::login))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
