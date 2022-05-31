use crate::domain::{playing, user};
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

impl From<user::User> for User {
    fn from(user: user::User) -> Self {
        Self { id: user.id, name: user.name }
    }
}

#[derive(Debug, Serialize)]
pub struct Playing {
    pub id: i32,
    pub name: String,
    pub discoverer: User,
    pub latitude: f64,
    pub longitude: f64,
    pub create_on: NaiveDateTime,
    pub update_on: NaiveDateTime,
    pub distance: f64,
}

impl From<(playing::Playing, f64)> for Playing {
    fn from((playing, distance): (playing::Playing, f64)) -> Self {
        Self {
            id: playing.id,
            name: playing.name,
            discoverer: playing.discoverer.into(),
            latitude: playing.latitude,
            longitude: playing.longitude,
            create_on: playing.create_on,
            update_on: playing.update_on,
            distance: distance,
        }
    }
}
