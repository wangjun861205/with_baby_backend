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

#[derive(Debug, Serialize, Deserialize, Queryable, QueryableByName, AsChangeset, Clone)]
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
