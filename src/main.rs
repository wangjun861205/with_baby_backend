mod domain;
mod handler;
mod persister;
mod schema;
mod token;

#[macro_use]
extern crate diesel;
extern crate serde;
use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use env_logger;
use token::jwt::JWT;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    HttpServer::new(move || {
        let mgr: ConnectionManager<PgConnection> = diesel::r2d2::ConnectionManager::new(
            "postgres://postgres:postgres@localhost/with_baby",
        );
        let pool = Pool::new(mgr).expect("failed to create database connection pool");
        let jwt = JWT::new("123456", chrono::Duration::days(30));
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(Data::new(handler::RandomGenerator::new()))
            .app_data(Data::new(handler::Hasher::new()))
            .app_data(Data::new(pool))
            .app_data(Data::new(jwt.clone()))
            .service(
                web::scope("")
                    .route(
                        "signup",
                        web::post()
                            .to(handler::user::signup::<handler::RandomGenerator, handler::Hasher>),
                    )
                    .route(
                        "signin",
                        web::post().to(handler::user::signin::<handler::Hasher, token::jwt::JWT>),
                    ),
            )
            .service(web::scope("api").wrap(jwt))
    })
    .bind(("localhost", 8000))?
    .run()
    .await
}
