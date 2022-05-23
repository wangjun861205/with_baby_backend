mod domain;
mod handler;
mod persister;
mod schema;
mod token;

#[macro_use]
extern crate diesel;
extern crate serde;
use actix_web::{App, HttpServer};
use token::jwt::JWT;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let jwt = JWT::new("123456", chrono::Duration::days(30));
    HttpServer::new(move || App::new().wrap(jwt.clone()))
        .bind(("localhost", 8000))?
        .run()
        .await
}
