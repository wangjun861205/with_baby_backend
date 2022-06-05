use super::{Error, PgPool};
use crate::domain::{
    outcomes::Playing,
    playing::{self, create_playing, Creation},
};
use crate::persister::postgres::PostgresPersister;
use crate::response::ListResponse;
use crate::token::UID;
use actix_web::{
    web::{get, post, resource, Data, Json, Query},
    Resource,
};
use anyhow::Context;
use serde::{Deserialize, Serialize};

pub fn register_router() -> Resource {
    resource("/playings").route(get().to(nearby)).route(post().to(create))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NearbyParams {
    latitude: f64,
    longitude: f64,
    page: i64,
    size: i64,
}

pub async fn nearby(Query(NearbyParams { latitude, longitude, page, size }): Query<NearbyParams>, db: Data<PgPool>) -> Result<Json<ListResponse<Playing>>, Error> {
    let err_ctx: &str = "failed to list nearby playings";
    let persister = PostgresPersister::new(db.get().context(err_ctx)?);
    let (l, total) = playing::nearby_playings(persister, latitude, longitude, (3 * 10i32.pow(4)) as f64, page, size).context(err_ctx)?;
    Ok(Json(ListResponse::new(l, total)))
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRequest {
    name: String,
    latitude: f64,
    longitude: f64,
    images: Vec<i32>,
}

pub async fn create(UID(uid): UID, Json(req): Json<CreateRequest>, db: Data<PgPool>) -> Result<Json<i32>, Error> {
    let err_ctx: &str = "failed to create playing";
    let persister = PostgresPersister::new(db.get().context(err_ctx)?);
    let id = create_playing(
        persister,
        Creation {
            name: req.name,
            latitude: req.latitude,
            longitude: req.longitude,
            discoverer: uid,
            images: req.images,
        },
    )
    .context(err_ctx)?;
    Ok(Json(id))
}
