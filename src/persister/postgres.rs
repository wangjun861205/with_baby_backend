use crate::domain::{
    playing,
    user::{self, UserPersister},
};
use crate::schema::*;
use anyhow::{Context, Error};
use chrono::NaiveDateTime;
use diesel::{
    dsl::sql,
    pg::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
    sql_types::{BigInt, Double},
    ExpressionMethods, QueryDsl, RunQueryDsl,
};
use serde::Serialize;

#[derive(Debug, Clone, Queryable, QueryableByName)]
#[table_name = "users"]
pub struct User {
    id: i32,
    name: String,
    phone: String,
    password: String,
    salt: String,
    create_on: chrono::NaiveDateTime,
    update_on: chrono::NaiveDateTime,
}

impl From<User> for user::User {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            name: u.name,
            phone: u.phone,
            password: u.password,
            salt: u.salt,
            create_on: u.create_on,
            update_on: u.update_on,
        }
    }
}

pub struct PostgresPersister {
    conn: PooledConnection<ConnectionManager<PgConnection>>,
}

impl PostgresPersister {
    pub fn new(conn: PooledConnection<ConnectionManager<PgConnection>>) -> Self {
        Self { conn }
    }
}

impl UserPersister for PostgresPersister {
    fn delete_user(&self, id: i32) -> Result<usize, Error> {
        let res = diesel::delete(users::table.filter(users::id.eq(id)))
            .execute(&self.conn)
            .context("failed to delete user")?;
        Ok(res)
    }
    fn get_user(&self, id: i32) -> Result<user::User, Error> {
        let user: User = users::table
            .filter(users::id.eq(id))
            .get_result(&self.conn)
            .context("failed to get user")?;
        Ok(user.into())
    }
    fn get_user_by_phone(&self, phone: &str) -> Result<crate::domain::user::User, Error> {
        let user: User = users::table
            .filter(users::phone.eq(phone))
            .get_result(&self.conn)
            .context("failed to get user by phone")?;
        Ok(user.into())
    }
    fn insert_user(&self, user: crate::domain::user::Insert) -> Result<i32, Error> {
        diesel::insert_into(users::table)
            .values((
                users::name.eq(user.name),
                users::phone.eq(user.phone),
                users::password.eq(user.password),
                users::salt.eq(user.salt),
            ))
            .returning(users::id)
            .get_result(&self.conn)
            .context("failed to insert user")
    }
    fn query_user(
        &self,
        query: crate::domain::user::Query,
    ) -> Result<(Vec<crate::domain::user::User>, i64), Error> {
        let mut q = users::table.into_boxed();
        if let Some(name) = &query.name {
            q = q.filter(users::name.eq(name));
        }
        if let Some(phone) = &query.phone {
            q = q.filter(users::phone.eq(phone));
        }
        let total = q.count().get_result(&self.conn)?;
        let mut q = users::table.into_boxed();
        if let Some(name) = &query.name {
            q = q.filter(users::name.eq(name));
        }
        if let Some(phone) = &query.phone {
            q = q.filter(users::phone.eq(phone));
        }
        let l: Vec<User> = q.load(&self.conn)?;
        let l: Vec<user::User> = l.into_iter().map(user::User::from).collect();
        Ok((l, total))
    }
    fn update_user(&self, id: i32, user: crate::domain::user::Update) -> Result<usize, Error> {
        let affected = diesel::update(users::table)
            .filter(users::id.eq(id))
            .set((
                users::name.eq(user.name),
                users::phone.eq(user.phone),
                users::password.eq(user.password),
                users::salt.eq(user.salt),
            ))
            .execute(&self.conn)?;
        Ok(affected)
    }
}

#[derive(Debug, Clone, QueryableByName, Queryable)]
#[table_name = "playings"]
pub struct Playing {
    id: i32,
    name: String,
    latitude: f64,
    longitude: f64,
    discoverer: i32,
    create_on: NaiveDateTime,
    update_on: NaiveDateTime,
}

impl playing::PlayingPersister for PostgresPersister {
    fn nearby_playings(
        &self,
        latitude: f64,
        longitude: f64,
        distance: f64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<playing::Playing>, i64), Error> {
        let q = playings::table
            .inner_join(users::table)
            .filter(sql(&format!(
                "earth_box(ll_to_earch({}, {}), {}) @> ll_to_earch(playings.latitude, playings.longitude)",
                latitude, longitude, distance
            )))
            .order_by(sql::<Double>(&format!("earth_distance(ll_to_earth({}, {}), ll_to_earth(playings.latitude, playings.longitude))", latitude, longitude)));
        let count = q.clone().count().get_result(&self.conn)?;
        let l: Vec<(Playing, User)> = q.clone().limit(limit).offset(offset).load(&self.conn)?;
        let res: Vec<playing::Playing> = l
            .into_iter()
            .map(|(p, u)| playing::Playing {
                id: p.id,
                name: p.name,
                latitude: p.latitude,
                longitude: p.longitude,
                discoverer: u.into(),
                create_on: p.create_on,
                update_on: p.update_on,
            })
            .collect();
        Ok((res, count))
    }

    fn insert_playing(&self, p: playing::Creation) -> Result<i32, Error> {
        let id = diesel::insert_into(playings::table)
            .values((
                playings::name.eq(p.name),
                playings::latitude.eq(p.latitude),
                playings::longitude.eq(p.longitude),
                playings::discoverer.eq(p.discoverer),
            ))
            .returning(playings::id)
            .get_result(&self.conn)?;
        Ok(id)
    }
}
