mod domain;
mod generator;
mod handler;
mod hasher;
mod persister;
mod response;
mod schema;
mod token;

#[macro_use]
extern crate diesel;
extern crate serde;
use actix_web::{
    middleware::Logger,
    web::{self, Data},
    web::{get, post, scope},
    App, HttpServer,
};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use env_logger;
use generator::random::Generator;
use hasher::sha::Hasher;
use rand::{rngs::ThreadRng, thread_rng};
use token::jwt::JWT;

const DATABASE_URL: &str = "DATABASE_URL";
const JWT_SECRET: &str = "JWT_SECRET";
const JWT_TOKEN_DURATION: &str = "JWT_TOKEN_DURATION";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("failed to load .env file");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    HttpServer::new(move || {
        let mgr: ConnectionManager<PgConnection> = diesel::r2d2::ConnectionManager::new(dotenv::var(DATABASE_URL).expect("DATABASE_URL environment variable not exists"));
        let pool = Pool::new(mgr).expect("failed to create database connection pool");
        let jwt = JWT::new(
            &dotenv::var(JWT_SECRET).expect("JWT_SECRET environment variable not exists"),
            chrono::Duration::days(
                dotenv::var(JWT_TOKEN_DURATION)
                    .expect("JWT_TOKEN_DURATION environment variable not exists")
                    .parse::<i64>()
                    .expect("JWT_TOKEN_DURATION environment variable must be integer"),
            ),
        );
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(Data::new(Generator::new(thread_rng())))
            .app_data(Data::new(Hasher::new()))
            .app_data(Data::new(pool))
            .app_data(Data::new(jwt.clone()))
            .service(
                scope("/user")
                    .route("/signup", web::post().to(handler::user::signup::<Generator<ThreadRng>, Hasher>))
                    .route("/signin", web::post().to(handler::user::signin::<Hasher, token::jwt::JWT>)),
            )
            .service(
                scope("/api")
                    .wrap(jwt)
                    .service(scope("/playings").route("", post().to(handler::playing::create)).route("", get().to(handler::playing::nearby))),
            )
    })
    .bind((
        dotenv::var("ADDRESS").expect("ADDRESS environment not exists"),
        dotenv::var("PORT").expect("PORT environment not exists").parse::<u16>().expect("port must be integer"),
    ))?
    .run()
    .await
}
