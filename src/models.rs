use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::{sql_types::Decimal, AsChangeset, Associations, Identifiable, Insertable, Queryable, QueryableByName};
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, QueryableByName, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub password: String,
    pub salt: String,
    pub create_on: NaiveDateTime,
    pub update_on: NaiveDateTime,
    pub avatar: Option<i32>,
}

#[derive(Debug, AsChangeset, Default)]
#[table_name = "users"]
pub struct UserCommand {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub password: Option<String>,
    pub salt: Option<String>,
    pub avatar: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, QueryableByName)]
#[table_name = "comments"]
pub struct Comment {
    id: i32,
    rank: i32,
    content: String,
    user: i32,
    location: i32,
    create_on: NaiveDateTime,
    update_on: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable, QueryableByName, AsChangeset)]
#[table_name = "comments"]
pub struct CommentCommand {
    rank: i32,
    content: String,
    user: i32,
    location: i32,
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

#[derive(Debug, Serialize, Deserialize, AsChangeset, Clone)]
#[table_name = "locations"]
pub struct LocationUpdating {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub category: i32,
    pub description: String,
    pub discoverer: i32,
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

#[derive(Debug, Serialize, Deserialize, Queryable, QueryableByName, Identifiable)]
#[table_name = "memories"]
pub struct Memory {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub owner: i32,
    pub location: i32,
    pub create_on: NaiveDateTime,
    pub update_on: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "memories"]
pub struct MemoryCommand {
    pub title: String,
    pub content: String,
    pub owner: i32,
    pub location: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryQuery {
    pub title: Option<String>,
    pub owner: Option<i32>,
    pub location: Option<i32>,
    pub create_before: Option<NaiveDateTime>,
    pub create_after: Option<NaiveDateTime>,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Serialize, Deserialize, Queryable, QueryableByName, Associations, Identifiable)]
#[table_name = "memory_upload_rels"]
#[belongs_to(Memory, foreign_key = "memory")]
#[belongs_to(Upload, foreign_key = "upload")]
pub struct MemoryUploadRel {
    pub id: i32,
    pub memory: i32,
    pub upload: i32,
}

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, QueryableByName, Associations)]
#[table_name = "equipments"]
#[belongs_to(Location, foreign_key = "location")]
pub struct Equipment {
    pub id: i32,
    pub name: String,
    pub is_required: bool,
    pub usage: String,
    pub location: i32,
    pub create_on: NaiveDateTime,
    pub update_on: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "equipments"]
pub struct EquipmentCommand {
    name: String,
    is_required: bool,
    usage: String,
    location: i32,
}
