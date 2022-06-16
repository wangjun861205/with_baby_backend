use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Queryable, QueryableByName};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, QueryableByName, AsChangeset)]
pub struct User {
    id: Option<i32>,
    name: Option<String>,
    phone: Option<String>,
    password: Option<String>,
    create_on: Option<NaiveDateTime>,
    update_on: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, QueryableByName, AsChangeset)]
pub struct Equipment {
    id: Option<i32>,
    name: Option<String>,
    is_required: Option<String>,
    usage: Option<String>,
    location: Option<i32>,
    create_on: Option<NaiveDateTime>,
    update_on: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, QueryableByName, AsChangeset)]
pub struct Comment {
    id: Option<i32>,
    rank: Option<i32>,
    content: Option<String>,
    user: Option<i32>,
    location: Option<i32>,
    create_on: Option<NaiveDateTime>,
    update_on: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, QueryableByName, AsChangeset)]
pub struct Location {
    id: Option<i32>,
    name: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    category: Option<i32>,
    description: Option<String>,
    discoverer: Option<i32>,
    distance: Option<f64>,
    create_on: Option<NaiveDateTime>,
    update_on: Option<NaiveDateTime>,
}
