use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, body, HttpRequest};
mod users;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/create", web::post().to(users::routs::post_create_user))
            .route("/db", web::get().to(users::routs::create_data_base))
            .route("/add", web::post().to(users::routs::post_add_user))
            .route("/get", web::post().to(users::routs::get_get_user))
    })
    .bind(("127.0.0.1", 9997))?
    .run()
    .await
}
