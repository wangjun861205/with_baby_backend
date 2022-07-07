// use super::models::Location;
use crate::error::Error;
use crate::geo::h3;
use crate::handlers::PgPool;
use crate::response::ListResponse;
use crate::serde::Deserialize;
use crate::token::UID;
use crate::{
    dao::{equipment, location, rank_aggregation, upload, user},
    models::{Equipment, Location, LocationInsertion, LocationUpdating, RankAggregation, RankAggregationInsert, Upload, User},
};

use actix_web::{
    web::{get, post, put, Data, Json, Path, Query},
    Scope,
};
use anyhow::Context;
use diesel::Connection;
use itertools::izip;
use std::default::Default;

pub fn register(scope: Scope) -> Scope {
    scope
        .route("", get().to(nearby_locations))
        .route("", post().to(create_location))
        .route("/{id}", get().to(detail))
        .route("/{id}", put().to(update))
        .route("/my", get().to(my_locations))
}

#[derive(Debug, Deserialize)]
pub struct NearbyRequest {
    latitude: f64,
    longitude: f64,
    limit: i64,
    offset: i64,
}

pub async fn nearby_locations(pool: Data<PgPool>, Query(params): Query<NearbyRequest>) -> Result<Json<ListResponse<(Location, User, Vec<Equipment>, Vec<Upload>, f64, RankAggregation)>>, Error> {
    let conn = pool.get().context("failed to get nearby locations")?;
    let ((locs, dists), total) = location::query(
        &conn,
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
    let users = user::discoverers_of_locations(&conn, &locs)?;
    let equips = equipment::equipements_of_locations(&conn, &locs)?;
    let uploads = upload::uploads_of_locations(&conn, &locs)?;
    let rank_agg = rank_aggregation::rank_aggs_of_location(&conn, &locs)?;
    Ok(Json(ListResponse::new(izip!(locs, users, equips, uploads, dists, rank_agg).collect(), total)))
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
    let conn = pool.get().context("failed to create location")?;
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
                geo_index: h3::index(body.latitude, body.longitude, 13),
            },
        )?;
        for img_id in body.images {
            upload::insert_location_upload_rel(&conn, upload::LocationUploadRelInsertion { location_id: id, upload_id: img_id })?;
        }
        // create rank aggregation
        rank_aggregation::insert(&conn, RankAggregationInsert { total: 0, count: 0, location_id: id })?;

        Ok(id)
    })?;
    Ok(Json(id))
}

#[derive(Debug, Deserialize)]
pub struct DetailParams {
    latitude: f64,
    longitude: f64,
}

pub async fn detail(pool: Data<PgPool>, id: Path<(i32,)>, Query(DetailParams { latitude, longitude }): Query<DetailParams>) -> Result<Json<(Location, User, Vec<Equipment>, Vec<Upload>, f64)>, Error> {
    let conn = pool.get().context("failed to get location detail")?;
    let (loc, dist) = location::get(&conn, id.0, latitude, longitude)?;
    let user = user::discoverer_of_location(&conn, &loc)?;
    let uploads = upload::uploads_of_location(&conn, &loc)?;
    let equipments = equipment::equipements_of_location(&conn, &loc)?;
    Ok(Json((loc, user, equipments, uploads, dist)))
}

#[derive(Debug, Deserialize)]
pub struct UpdateBody {
    name: String,
    description: String,
    category: i32,
    images: Vec<i32>,
}

pub async fn update(pool: Data<PgPool>, uid: UID, id: Path<(i32,)>, Json(body): Json<UpdateBody>) -> Result<Json<usize>, Error> {
    let conn = pool.get().context("failed to update location")?;
    let (loc, user, _) = location::get_without_coord(&conn, id.0)?;
    if user.id != uid.0 {
        return Err(Error::PermissionError);
    }
    conn.transaction::<(), anyhow::Error, _>(|| {
        location::update(
            &conn,
            id.0,
            LocationUpdating {
                name: body.name,
                latitude: loc.latitude,
                longitude: loc.longitude,
                category: body.category,
                description: body.description,
                discoverer: user.id,
                geo_index: loc.geo_index,
            },
        )?;
        location::clear_images(&conn, id.0)?;
        location::add_images(&conn, id.0, body.images)?;
        Ok(())
    })?;
    Ok(Json(1))
}

#[derive(Debug, Deserialize)]
pub struct MyLocations {
    latitude: f64,
    longitude: f64,
    limit: i64,
    offset: i64,
    order_by: location::OrderBy,
}

pub(crate) async fn my_locations(pool: Data<PgPool>, UID(uid): UID, Query(q): Query<MyLocations>) -> Result<Json<ListResponse<(Location, f64)>>, Error> {
    let ((locs, dists), total) = location::query(
        &pool.get()?,
        location::Query {
            latitude: q.latitude,
            longitude: q.longitude,
            radius: 10i32.pow(5) as f64,
            limit: q.limit,
            offset: q.offset,
            order_by: q.order_by,
            ..Default::default()
        },
    )?;
    let list = izip!(locs, dists).into_iter().collect();
    Ok(Json(ListResponse::new(list, total)))
}
