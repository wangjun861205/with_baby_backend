use super::{Error, PgPool};
use crate::domain::playing::{self, create_playing, Creation, Playing};
use crate::persister::postgres::PostgresPersister;
use crate::response::ListResponse;
use crate::token::UID;
use actix_web::{
    web::{Data, Json, Query},
    HttpRequest,
};
use anyhow::Context;
use serde::{Deserialize, Serialize};

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
    let (l, total) = playing::nearby_playings(persister, latitude, longitude, 10f64, page, size).context(err_ctx)?;
    Ok(Json(ListResponse::new(l, total)))
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRequest {
    name: String,
    latitude: f64,
    longitude: f64,
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
        },
    )
    .context(err_ctx)?;
    Ok(Json(id))
}
