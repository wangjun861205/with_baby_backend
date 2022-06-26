use crate::models;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Serialize, Queryable)]
pub struct Upload {
    pub id: i32,
    pub fetch_code: String,
    pub owner: i32,
    pub create_on: NaiveDateTime,
    pub update_on: NaiveDateTime,
}

impl From<models::Upload> for Upload {
    fn from(u: models::Upload) -> Self {
        Self {
            id: u.id,
            fetch_code: u.fetch_code,
            owner: u.owner,
            create_on: u.create_on,
            update_on: u.update_on,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct User {
    id: i32,
    name: String,
}

impl From<models::User> for User {
    fn from(u: models::User) -> Self {
        Self { id: u.id, name: u.name }
    }
}

#[derive(Debug, Serialize)]
pub struct Equipment {
    pub id: i32,
    pub name: String,
    pub is_required: bool,
    pub usage: String,
    pub location: i32,
    pub create_on: NaiveDateTime,
    pub update_on: NaiveDateTime,
}

impl From<models::Equipment> for Equipment {
    fn from(equip: models::Equipment) -> Self {
        Self {
            id: equip.id,
            name: equip.name,
            is_required: equip.is_required,
            usage: equip.usage,
            location: equip.location,
            create_on: equip.create_on,
            update_on: equip.update_on,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Location {
    pub id: i32,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub category: i32,
    pub description: String,
    pub discoverer: User,
    pub create_on: NaiveDateTime,
    pub update_on: NaiveDateTime,
    pub distance: f64,
    pub images: Vec<Upload>,
    pub equipments: Vec<Equipment>,
}

impl From<(models::Location, models::User, f64, Vec<models::Upload>, Vec<models::Equipment>)> for Location {
    fn from((l, u, d, ims, eqps): (models::Location, models::User, f64, Vec<models::Upload>, Vec<models::Equipment>)) -> Self {
        Self {
            id: l.id,
            name: l.name,
            latitude: l.latitude,
            longitude: l.longitude,
            category: l.category,
            description: l.description,
            discoverer: u.into(),
            create_on: l.create_on,
            update_on: l.update_on,
            distance: d,
            images: ims.into_iter().map(|im| im.into()).collect(),
            equipments: eqps.into_iter().map(|v| v.into()).collect(),
        }
    }
}
