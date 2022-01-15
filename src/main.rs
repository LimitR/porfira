use actix_web::{body, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
mod users;
use postgres::{Client, NoTls, Statement};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/create", web::post().to(users::routs::post_create_user))
            .route("/db", web::post().to(users::routs::create_data_base))
            .route("/add", web::post().to(users::routs::post_add_user))
            .route("/get/id", web::post().to(users::routs::get_get_user_i32))
            .route("/get/uuid", web::post().to(users::routs::get_get_user_uuid))
    })
    .bind(("127.0.0.1", 9997))?
    .run()
    .await
}
