mod domain;
mod persister;
mod schema;
mod handler;
mod token;



#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("login", web::get().user::login)
            .route("signup", web::domain::user::signup)
    })
    .bind(("localhost", 8000))?
    .run()
    .await
}
