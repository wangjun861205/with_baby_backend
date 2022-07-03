use super::{PgPool, QueryResponse};
use crate::dao::memory::{add_images, find, insert, query_images};
use crate::error::Error;
use crate::models::{Location, Memory, MemoryCommand, MemoryOrderBy, MemoryQuery, Upload};
use crate::response::ListResponse;
use crate::token::UID;
use actix_web::{
    web::{get, post, Data, Json, Path, Query},
    Scope,
};
use anyhow::Context;
use diesel::Connection;
use itertools::izip;
use serde::Deserialize;
use std::default::Default;

pub fn register(scope: Scope) -> Scope {
    scope.route("/{id}/memories", post().to(create)).route("/{id}/memories", get().to(list)).route("/my", get().to(my))
}

#[derive(Debug, Deserialize)]
pub struct CreateBody {
    pub title: String,
    pub content: String,
    pub images: Vec<i32>,
}
pub async fn create(pool: Data<PgPool>, uid: UID, location: Path<(i32,)>, Json(body): Json<CreateBody>) -> Result<Json<i32>, Error> {
    let conn = pool.get().context("failed to create memory")?;
    let id = conn.transaction::<i32, anyhow::Error, _>(|| {
        let id = insert(
            &conn,
            MemoryCommand {
                title: body.title,
                content: body.content,
                owner: uid.0,
                location: location.0,
            },
        )?;
        add_images(&conn, id, body.images)?;
        Ok(id)
    })?;
    Ok(Json(id))
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    latitude: f64,
    longitude: f64,
    limit: i64,
    offset: i64,
    order_by: MemoryOrderBy,
}

pub async fn list(
    pool: Data<PgPool>,
    location: Path<(i32,)>,
    Query(ListParams {
        latitude,
        longitude,
        order_by,
        limit,
        offset,
    }): Query<ListParams>,
) -> Result<Json<ListResponse<(Memory, Location, Vec<Upload>, f64)>>, Error> {
    let conn = pool.get().context("failed to list memories")?;
    let (list, total) = conn.transaction::<(Vec<(Memory, Location, Vec<Upload>, f64)>, i64), anyhow::Error, _>(|| {
        let (m, l, d, total) = find(
            &conn,
            MemoryQuery {
                title: None,
                owner: None,
                location: Some(location.0),
                create_before: None,
                create_after: None,
                limit: limit,
                offset: offset,
                latitude: latitude,
                longitude: longitude,
                order_by: order_by,
            },
        )?;
        let imgs = query_images(&conn, &m)?;
        Ok((izip!(m, l, imgs, d).collect(), total))
    })?;
    Ok(Json(ListResponse::new(list, total)))
}

#[derive(Debug, Deserialize)]
pub struct My {
    limit: i64,
    offset: i64,
    title: Option<String>,
    latitude: f64,
    longitude: f64,
    order_by: MemoryOrderBy,
}

pub async fn my(pool: Data<PgPool>, UID(uid): UID, Query(q): Query<My>) -> Result<Json<ListResponse<(Memory, Location, f64)>>, Error> {
    let (mems, locs, dists, total) = find(
        &pool.get()?,
        MemoryQuery {
            latitude: q.latitude,
            longitude: q.longitude,
            title: q.title,
            owner: Some(uid),
            location: None,
            create_before: None,
            create_after: None,
            limit: q.limit,
            offset: q.offset,
            order_by: q.order_by,
        },
    )?;
    Ok(Json(ListResponse::new(izip!(mems, locs, dists).collect(), total)))
}

#[derive(Debug, Deserialize)]
pub struct NearMemories {
    latitude: f64,
    longitude: f64,
    limit: i64,
    offset: i64,
}

pub async fn near_memories(
    pool: Data<PgPool>,
    Query(NearMemories { latitude, longitude, limit, offset }): Query<NearMemories>,
) -> Result<Json<ListResponse<(Memory, Location, Vec<Upload>, f64)>>, Error> {
    let conn = pool.get()?;
    let (mems, locs, dists, total) = find(
        &conn,
        MemoryQuery {
            latitude: latitude,
            longitude: longitude,
            limit: limit,
            offset: offset,
            order_by: MemoryOrderBy::Distance,
            ..Default::default()
        },
    )?;
    let imgs = query_images(&conn, &mems)?;
    let list = izip!(mems, locs, imgs, dists).collect();
    Ok(Json(ListResponse::new(list, total)))
}
