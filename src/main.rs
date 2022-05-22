mod domain;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("login", web::domain::user::login))
        .bind(("localhost", 8000))?
        .run()
        .await
}
