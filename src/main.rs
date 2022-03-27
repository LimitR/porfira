extern crate argon2;
extern crate core;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::path::Path;
use std::time::Duration;

use actix_web::{body, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
mod ability;
mod plugins;
mod users;
mod db;
mod models;

use sqlx::{PgPool, Pool, Postgres};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = db_pool().await;
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/registration", web::post().to(users::routs::registration))
            .route("/login", web::post().to(users::routs::login))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}


pub async fn db_pool() -> Pool<Postgres> {
    Pool::<Postgres>::connect(&dotenv::var("URL_DB").unwrap()).await.unwrap()
}