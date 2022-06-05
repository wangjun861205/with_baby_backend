use crate::domain::{
    playing,
    upload::{self, Insertion, UploadPersister},
    user::{self, UserPersister},
};
use crate::schema::*;
use anyhow::{Context, Error};
use chrono::NaiveDateTime;
use diesel::{
    dsl::{exists, select, sql},
    pg::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
    sql_types::Double,
    Associations, BelongingToDsl, ExpressionMethods, GroupedBy, QueryDsl, RunQueryDsl,
};

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
        let res = diesel::delete(users::table.filter(users::id.eq(id))).execute(&self.conn).context("failed to delete user")?;
        Ok(res)
    }
    fn get_user(&self, id: i32) -> Result<user::User, Error> {
        let user: User = users::table.filter(users::id.eq(id)).get_result(&self.conn).context("failed to get user")?;
        Ok(user.into())
    }
    fn get_user_by_phone(&self, phone: &str) -> Result<crate::domain::user::User, Error> {
        let user: User = users::table.filter(users::phone.eq(phone)).get_result(&self.conn).context("failed to get user by phone")?;
        Ok(user.into())
    }
    fn insert_user(&self, user: crate::domain::user::Insert) -> Result<i32, Error> {
        diesel::insert_into(users::table)
            .values((users::name.eq(user.name), users::phone.eq(user.phone), users::password.eq(user.password), users::salt.eq(user.salt)))
            .returning(users::id)
            .get_result(&self.conn)
            .context("failed to insert user")
    }
    fn query_user(&self, query: crate::domain::user::Query) -> Result<(Vec<crate::domain::user::User>, i64), Error> {
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
            .set((users::name.eq(user.name), users::phone.eq(user.phone), users::password.eq(user.password), users::salt.eq(user.salt)))
            .execute(&self.conn)?;
        Ok(affected)
    }

    fn exists_user_by_phone(&self, phone: &str) -> Result<bool, Error> {
        let exists = select(exists(users::table.filter(users::phone.eq(phone)))).get_result::<bool>(&self.conn)?;
        Ok(exists)
    }
}

#[derive(Identifiable, Debug, Clone, QueryableByName, Queryable, PartialEq)]
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
    fn nearby_playings(&self, latitude: f64, longitude: f64, distance: f64, limit: i64, offset: i64) -> Result<(Vec<(playing::Playing, f64)>, i64), Error> {
        let q = playings::table.inner_join(users::table).filter(sql(&format!(
            "earth_box(ll_to_earth({}, {}), {}) @> ll_to_earth(playings.latitude, playings.longitude)",
            latitude, longitude, distance
        )));
        let count = q.clone().count().get_result(&self.conn)?;
        let q = q
            .clone()
            .select((
                playings::all_columns,
                users::all_columns,
                sql::<Double>(&format!(
                    "earth_distance(ll_to_earth({}, {}), ll_to_earth(playings.latitude, playings.longitude)) as distance",
                    latitude, longitude
                )),
            ))
            .order_by(sql::<Double>("distance"));
        let l: Vec<(Playing, User, f64)> = q.clone().limit(limit).offset(offset).load(&self.conn)?;
        let ps: Vec<Playing> = l.iter().map(|(p, _, _)| p.clone()).collect();
        let uploads: Vec<Vec<(PlayingsUpload, Upload)>> = PlayingsUpload::belonging_to(&ps).inner_join(uploads::table).load(&self.conn)?.grouped_by(&ps);
        let res: Vec<(playing::Playing, f64)> = l
            .into_iter()
            .zip(uploads)
            .map(|((p, u, distance), images)| {
                (
                    playing::Playing {
                        id: p.id,
                        name: p.name,
                        latitude: p.latitude,
                        longitude: p.longitude,
                        discoverer: u.into(),
                        images: images.into_iter().map(|(_, u)| u.into()).collect(),
                        create_on: p.create_on,
                        update_on: p.update_on,
                    },
                    distance,
                )
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
        for img in p.images {
            diesel::insert_into(playings_uploads::table)
                .values((playings_uploads::playing_id.eq(id), playings_uploads::upload_id.eq(img)))
                .execute(&self.conn)?;
        }
        Ok(id)
    }
}

#[derive(Queryable)]
struct Upload {
    id: i32,
    fetch_code: String,
    owner: i32,
    create_on: NaiveDateTime,
    update_on: NaiveDateTime,
}

impl Into<upload::Upload> for Upload {
    fn into(self) -> upload::Upload {
        upload::Upload {
            id: self.id,
            fetch_code: self.fetch_code,
            owner: self.owner,
            create_on: self.create_on,
            update_on: self.update_on,
        }
    }
}

impl UploadPersister for PostgresPersister {
    fn insert_upload(&self, ins: Insertion) -> Result<i32, Error> {
        let id = diesel::insert_into(uploads::table)
            .values((uploads::fetch_code.eq(ins.fetch_code), uploads::owner.eq(ins.owner)))
            .returning(uploads::id)
            .get_result(&self.conn)?;
        Ok(id)
    }

    fn get_upload(&self, id: i32) -> Result<upload::Upload, Error> {
        let u: Upload = uploads::table.find(id).get_result(&self.conn)?;
        Ok(u.into())
    }

    fn query_upload_by_ids(&self, ids: &Vec<i32>) -> Result<Vec<upload::Upload>, Error> {
        uploads::table
            .filter(uploads::id.eq_any(ids))
            .load::<Upload>(&self.conn)
            .map(|l| l.into_iter().map(|u| u.into()).collect())
            .map_err(|e| Error::from(e))
    }
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Playing)]
#[belongs_to(Upload)]
pub struct PlayingsUpload {
    id: i32,
    playing_id: i32,
    upload_id: i32,
}
