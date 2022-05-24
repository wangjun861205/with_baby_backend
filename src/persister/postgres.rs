use crate::domain::user;
use crate::domain::user::UserPersister;
use crate::schema::*;
use anyhow::{Context, Error};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
    ExpressionMethods, QueryDsl, RunQueryDsl,
};

#[derive(Queryable)]
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
