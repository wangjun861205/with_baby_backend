use super::models::Location;
use super::Error;
use crate::handlers::PgPool;
use crate::response::ListResponse;
use crate::serde::Deserialize;
use crate::token::UID;
use crate::{
    dao::{location, upload},
    models::LocationInsertion,
};
use actix_web::{
    web::{get, post, Data, Json, Query},
    Scope,
};
use anyhow::Context;
use diesel::Connection;
use std::default::Default;

pub fn register(scope: &str) -> Scope {
    Scope::new(scope).route("/", get().to(nearby_locations)).route("/", post().to(create_location))
}

#[derive(Debug, Deserialize)]
pub struct NearbyRequest {
    latitude: f64,
    longitude: f64,
    limit: i64,
    offset: i64,
}

pub async fn nearby_locations(pool: Data<PgPool>, Query(params): Query<NearbyRequest>) -> Result<Json<ListResponse<Location>>, Error> {
    let (list, total) = location::find(
        &pool.get()?,
        location::Query {
            latitude: params.latitude,
            longitude: params.longitude,
            radius: 10i32.pow(5) as f64,
            limit: if params.limit > 40 { 40 } else { params.limit },
            offset: params.offset,
            ..Default::default()
        },
    )
    .context("failed to find nearby locations")?;
    Ok(Json(ListResponse::new(list.into_iter().map(|v| v.into()).collect(), total)))
}

#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub category: i32,
    pub description: String,
    pub images: Vec<i32>,
}

pub async fn create_location(pool: Data<PgPool>, uid: UID, Json(body): Json<CreateRequest>) -> Result<Json<i32>, Error> {
    let conn = pool.get()?;
    let id = conn.transaction(|| {
        let exists = location::exists(
            &conn,
            location::Query {
                latitude: body.latitude,
                longitude: body.longitude,
                radius: 500.0,
                ..Default::default()
            },
        )?;
        if exists {
            return Err(anyhow::Error::msg("location already exists"));
        }
        let id = location::insert(
            &conn,
            LocationInsertion {
                name: body.name,
                latitude: body.latitude,
                longitude: body.longitude,
                category: body.category,
                description: body.description,
                discoverer: uid.0,
            },
        )?;
        for img_id in body.images {
            upload::insert_location_upload_rel(&conn, upload::LocationUploadRelInsertion { location_id: id, upload_id: img_id })?;
        }
        Ok(id)
    })?;
    Ok(Json(id))
}
