extern crate argon2;
extern crate core;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::path::Path;
use std::time::Duration;

use actix_web::{body, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
mod ability;
mod db;
mod models;
mod plugins;
mod users;

use sqlx::{PgPool, Pool, Postgres};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let mut pool = db_pool().await;
    println!("http://{}:{}", dotenv::var("URL").unwrap(), dotenv::var("PORT").unwrap().parse::<i32>().unwrap());
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/registration", web::post().to(users::routs::registration))
            .route("/login", web::post().to(users::routs::login))
    })
    .bind((dotenv::var("URL").unwrap().as_ref(), dotenv::var("PORT").unwrap().parse::<i32>().unwrap()))?
    .run()
    .await
}

pub async fn db_pool() -> Pool<Postgres> {
    Pool::<Postgres>::connect(&dotenv::var("URL_DB").unwrap())
        .await.unwrap()

}
