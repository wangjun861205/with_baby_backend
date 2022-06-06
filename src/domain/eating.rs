use crate::domain::{
    upload::{Upload, UploadPersister},
    user::{User, UserPersister},
};
use anyhow::{Context, Error};
use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct Eating {
    pub id: i32,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub discoverer: i32,
    pub create_on: NaiveDateTime,
    pub update_on: NaiveDateTime,
}

pub struct QueryByDistance {
    pub latitude: f64,
    pub longitude: f64,
    pub distance: f64,
}

pub struct Creation {
    pub latitude: f64,
    pub longitude: f64,
    pub discoverer: i32,
    pub images: Vec<i32>,
}

pub trait EatingPersister {
    fn query_eating_by_distance(&self, query: QueryByDistance) -> Result<Vec<Eating>, Error>;
    fn insert_eating(&self, creation: Creation) -> Result<i32, Error>;
}

pub fn nearby_eatings<P>(persister: P, query: QueryByDistance) -> Result<Vec<(Eating, User, Vec<Upload>)>, Error>
where
    P: EatingPersister + UserPersister + UploadPersister,
{
    let eatings = persister.query_eating_by_distance(query)?;
    let discoverer_ids: Vec<i32> = eatings.iter().map(|e| e.discoverer).collect();
    let discoverers = persister.query_user_by_ids(discoverer_ids)?;
    let images: Vec<Vec<Upload>> = persister.query_upload_by_eatings(&eatings)?;
    Ok(eatings.into_iter().zip(discoverers).zip(images).map(|((e, d), i)| (e, d, i)).collect())
}
