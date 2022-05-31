use crate::domain::{playing, user};
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct User {
    pub name: String,
}

impl From<user::User> for User {
    fn from(u: user::User) -> Self {
        Self { name: u.name }
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
}

impl From<playing::Playing> for Playing {
    fn from(p: playing::Playing) -> Self {
        Self {
            id: p.id,
            name: p.name,
            discoverer: p.discoverer.into(),
            latitude: p.latitude,
            longitude: p.longitude,
            create_on: p.create_on,
            update_on: p.update_on,
        }
    }
}
