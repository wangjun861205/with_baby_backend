use crate::domain::eating;
use crate::handlers::user;
use crate::handlers::Error;
use crate::handlers::PgPool;
use crate::handlers::QueryResponse;
use crate::persister::postgres::PostgresPersister;
use crate::token::UID;
use actix_web::web::Json;
use chrono::NaiveDateTime;
use serde::Serialize;

pub struct CreateRequest {
    name: String,
    latitude: f64,
    longitude: f64,
    images: Vec<i32>,
}

pub async fn create_eating(UID(uid): UID, Json(req): Json<CreateRequest>, pool: PgPool) -> Result<i32, Error> {
    let persister = PostgresPersister::new(pool.get()?);
    let id = eating::create_eating(
        persister,
        eating::Insertion {
            name: req.name,
            latitude: req.latitude,
            longitude: req.longitude,
            discoverer: uid,
        },
        req.images,
    )?;
    Ok(id)
}

pub struct NearbyQuery {
    latitude: f64,
    longitude: f64,
}

#[derive(Debug, Serialize)]
pub struct Eating {
    id: i32,
    name: String,
    latitude: f64,
    longitude: f64,
    discoverer: user::User,
    create_on: NaiveDateTime,
    update_on: NaiveDateTime,
}

pub async fn nearby_eatings(Json(req): Json<NearbyQuery>, pool: PgPool) -> Result<QueryResponse<Eating>, Error> {}
