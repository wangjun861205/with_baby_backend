use super::user::User;
use crate::domain::{
    outcomes,
    upload::{Upload, UploadPersister},
};
use anyhow::{Context, Error};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Playing {
    pub id: i32,
    pub name: String,
    pub discoverer: User,
    pub latitude: f64,
    pub longitude: f64,
    pub images: Vec<Upload>,
    pub create_on: NaiveDateTime,
    pub update_on: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Creation {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub discoverer: i32,
    pub images: Vec<i32>,
}

pub trait PlayingPersister {
    fn nearby_playings(&self, latitude: f64, longitude: f64, distance: f64, limit: i64, offset: i64) -> Result<(Vec<(Playing, f64)>, i64), Error>;

    fn insert_playing(&self, playing: Creation) -> Result<i32, Error>;
}

pub fn nearby_playings<PP: PlayingPersister>(persister: PP, latitude: f64, longitude: f64, distance: f64, page: i64, size: i64) -> Result<(Vec<outcomes::Playing>, i64), Error> {
    persister
        .nearby_playings(latitude, longitude, distance, size, (page - 1) * size)
        .context("failed to get nearby playings")
        .map(|(playings, total)| (playings.into_iter().map(|v| v.into()).collect(), total))
}

pub fn create_playing<PP: PlayingPersister + UploadPersister>(persister: PP, playing: Creation) -> Result<i32, Error> {
    let images = persister.query_upload_by_ids(&playing.images)?;
    // 如果找不到全部用户所提供的图片则报错
    if images.len() != playing.images.len() {
        return Err(Error::msg("cannot find all images").context("failed to create playing"));
    }
    // 如果用户提供的图片包含不是属于他自己的图片则报错
    if images.into_iter().any(|u| u.owner != playing.discoverer) {
        return Err(Error::msg("no permission to use other's image").context("failed to create playing"));
    }
    persister.insert_playing(playing)
}
