use crate::domain::eating;
use crate::handlers::user;
use crate::handlers::Error;
use crate::handlers::PgPool;
use crate::handlers::QueryResponse;
use crate::persister::postgres::PostgresPersister;
use crate::token::UID;
use actix_web::{
    web::{get, post, Data, Json},
    Scope,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

pub fn register(scope: &str) -> Scope {
    Scope::new(scope).route("", get().to(nearby_eatings)).route("", post().to(create_eating))
}

#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    name: String,
    latitude: f64,
    longitude: f64,
    images: Vec<i32>,
}

pub async fn create_eating(UID(uid): UID, Json(req): Json<CreateRequest>, pool: Data<PgPool>) -> Result<Json<i32>, Error> {
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
    Ok(Json(id))
}

#[derive(Debug, Deserialize)]
pub struct NearbyQuery {
    latitude: f64,
    longitude: f64,
    page: i64,
    size: i64,
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
    images: Vec<i32>,
}

pub async fn nearby_eatings(Json(req): Json<NearbyQuery>, pool: Data<PgPool>) -> Result<Json<QueryResponse<Eating>>, Error> {
    let persister = PostgresPersister::new(pool.get()?);
    let (l, total) = eating::nearby_eatings(
        persister,
        eating::NearbyQuery {
            latitude: req.latitude,
            longitude: req.longitude,
            radius: (3 * 10i32.pow(4)) as f64,
            page: req.page,
            size: req.size,
        },
    )?;
    Ok(Json(QueryResponse::new(
        l.into_iter()
            .map(|(e, u, us)| Eating {
                id: e.id,
                name: e.name,
                latitude: e.latitude,
                longitude: e.longitude,
                discoverer: u.into(),
                create_on: e.create_on,
                update_on: e.update_on,
                images: us.into_iter().map(|v| v.id).collect(),
            })
            .collect(),
        total,
    )))
}
