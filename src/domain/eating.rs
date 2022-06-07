use crate::domain::{
    upload::{Upload, UploadPersister},
    user::{User, UserPersister},
};
use anyhow::{Context, Error};
use chrono::NaiveDateTime;

use super::upload;

#[derive(Debug, Clone)]
pub struct Eating {
    pub id: i32,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub discoverer: i32,
    pub create_on: NaiveDateTime,
    pub update_on: NaiveDateTime,
    pub distance: f64,
}

pub struct NearbyQuery {
    pub latitude: f64,
    pub longitude: f64,
    pub radius: f64,
    pub page: i64,
    pub size: i64,
}

pub struct QueryByDistance {
    pub latitude: f64,
    pub longitude: f64,
    pub radius: f64,
    pub limit: i64,
    pub offset: i64,
}

pub struct Insertion {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub discoverer: i32,
}

pub trait EatingPersister {
    fn query_eating_by_distance(&self, query: QueryByDistance) -> Result<(Vec<Eating>, i64), Error>;
    fn insert_eating(&self, creation: Insertion) -> Result<i32, Error>;
}

pub fn nearby_eatings<P>(persister: P, query: NearbyQuery) -> Result<(Vec<(Eating, User, Vec<Upload>)>, i64), Error>
where
    P: EatingPersister + UserPersister + UploadPersister,
{
    let (eatings, total) = persister.query_eating_by_distance(QueryByDistance {
        latitude: query.latitude,
        longitude: query.longitude,
        radius: query.radius,
        limit: query.size,
        offset: (query.page - 1) * query.size,
    })?;
    let discoverer_ids: Vec<i32> = eatings.iter().map(|e| e.discoverer).collect();
    let discoverers = persister.query_user_by_ids(discoverer_ids)?;
    let images: Vec<Vec<Upload>> = persister.query_upload_by_eatings(&eatings)?;
    Ok((eatings.into_iter().zip(discoverers).zip(images).map(|((e, d), i)| (e, d, i)).collect(), total))
}

pub fn create_eating<P>(persister: P, eating: Insertion, images: Vec<i32>) -> Result<i32, Error>
where
    P: EatingPersister + UploadPersister,
{
    let eating_id = persister.insert_eating(eating)?;
    persister.insert_eating_uploads(upload::EatingUploadsInsertion {
        eating_id: eating_id,
        upload_ids: images,
    })?;
    Ok(eating_id)
}
