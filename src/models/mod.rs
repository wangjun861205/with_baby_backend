use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Associations, BelongingToDsl, Identifiable, Insertable, Queryable, QueryableByName};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, QueryableByName, AsChangeset, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub password: String,
    pub salt: String,
    pub create_on: NaiveDateTime,
    pub update_on: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, QueryableByName, AsChangeset, Insertable)]
#[table_name = "equipments"]
pub struct Equipment {
    id: Option<i32>,
    name: Option<String>,
    is_required: Option<bool>,
    usage: Option<String>,
    location: Option<i32>,
    create_on: Option<NaiveDateTime>,
    update_on: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, QueryableByName, AsChangeset, Insertable)]
#[table_name = "comments"]
pub struct Comment {
    id: Option<i32>,
    rank: Option<i32>,
    content: Option<String>,
    user: Option<i32>,
    location: Option<i32>,
    create_on: Option<NaiveDateTime>,
    update_on: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "locations"]
pub struct LocationInsertion {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub category: i32,
    pub description: String,
    pub discoverer: i32,
}

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, QueryableByName, AsChangeset, Clone)]
#[table_name = "locations"]
pub struct Location {
    pub id: i32,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub category: i32,
    pub description: String,
    pub discoverer: i32,
    pub create_on: NaiveDateTime,
    pub update_on: NaiveDateTime,
}

#[derive(Debug, Serialize, Queryable)]
pub struct Upload {
    pub id: i32,
    pub fetch_code: String,
    pub owner: i32,
    pub create_on: NaiveDateTime,
    pub update_on: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Associations, Identifiable)]
#[belongs_to(Location)]
#[belongs_to(Upload)]
pub struct LocationUploadRel {
    pub id: i32,
    pub location_id: i32,
    pub upload_id: i32,
}
