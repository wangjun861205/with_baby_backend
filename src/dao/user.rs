use crate::models::{Location, User, UserCommand};
use crate::schema::*;
use anyhow::{Context, Error};
use diesel::sql_types::{Array, Integer};
use diesel::{dsl::sql, pg::Pg, sql_query, Connection, ExpressionMethods, QueryDsl, RunQueryDsl};

pub fn discoverers_of_locations<T>(conn: &T, locations: &Vec<Location>) -> Result<Vec<User>, Error>
where
    T: Connection<Backend = Pg>,
{
    sql_query("SELECT u.* FROM users AS u JOIN unnest($1) WITH ORDINALITY t(id, ord) ON u.id = t.id")
        .bind::<Array<Integer>, _>(locations.into_iter().map(|l| l.discoverer).collect::<Vec<i32>>())
        .load::<User>(conn)
        .context("failed to query discoverers of locations")
}

pub fn discoverer_of_location<T>(conn: &T, location: &Location) -> Result<User, Error>
where
    T: Connection<Backend = Pg>,
{
    users::table.filter(users::id.eq(location.discoverer)).first(conn).context("failed to get discoverer of location")
}

pub fn update<T>(conn: &T, id: i32, user: UserCommand) -> Result<usize, Error>
where
    T: Connection<Backend = Pg>,
{
    diesel::update(users::table.filter(users::id.eq(id))).set(user).execute(conn).context("failed to update user")
}
