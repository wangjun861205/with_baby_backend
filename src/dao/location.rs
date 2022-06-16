use crate::models::{Location, LocationInsertion, User};
use crate::schema::*;
use anyhow::{Context, Error};
use diesel::{dsl::sql, insert_into, pg::PgConnection, query_builder::SelectQuery, sql_types::Double, ExpressionMethods, QueryDsl, RunQueryDsl, TextExpressionMethods};
pub struct Query {
    name: Option<String>,
    latitude: f64,
    longitude: f64,
    radius: f64,
    category: Option<i32>,
    limit: i64,
    offset: i64,
}

pub fn find(conn: &PgConnection, query: Query) -> Result<(Vec<(Location, User, f64)>, i64), Error> {
    let mut c = locations::table.inner_join(users::table).into_boxed();
    let mut q = locations::table
        .inner_join(users::table)
        .select((
            locations::all_columns,
            users::all_columns,
            sql::<Double>(&format!(
                "earth_distance(ll_to_earth({}, {}), ll_to_earth(locations.latitude, locations.longitude)) as distance",
                query.latitude, query.longitude
            )),
        ))
        .limit(query.limit)
        .offset(query.offset)
        .into_boxed();
    c = c.filter(sql(&format!(
        "earth_box(ll_to_earth({}, {}), {}) @> ll_to_earth(locations.latitude, locations.longitude)",
        query.latitude, query.longitude, query.radius
    )));
    q = q.filter(sql(&format!(
        "earth_box(ll_to_earth({}, {}), {}) @> ll_to_earth(locations.latitude, locations.longitude)",
        query.latitude, query.longitude, query.radius
    )));
    if let Some(name) = query.name {
        c = c.filter(locations::name.like(format!("%{}%", name)));
        q = q.filter(locations::name.like(format!("%{}%", name)));
    }
    if let Some(category) = query.category {
        c = c.filter(locations::category.eq(category));
        q = q.filter(locations::category.eq(category));
    }
    let total = c.count().get_result(conn).context("failed to find locations")?;
    let l: Vec<(Location, User, f64)> = q.load(conn).context("failed to find locations")?;
    Ok((l, total))
}

pub fn get(id: i32) -> Result<Location, Error> {
    todo!()
}

pub fn insert(conn: &PgConnection, loc: LocationInsertion) -> Result<i32, Error> {
    insert_into(locations::table).values(loc).returning(locations::id).get_result(conn).context("failed to insert location")
}

pub fn update(loc: Location) -> Result<usize, Error> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::{find, insert, Query};
    use crate::diesel::Connection;
    use crate::models::LocationInsertion;
    #[test]
    fn test_find() {
        let conn = diesel::PgConnection::establish("postgres://postgres:postgres@localhost/with_baby").expect("failed to connect to database");
        // insert(
        //     &conn,
        //     LocationInsertion {
        //         name: "test".into(),
        //         latitude: 1.1,
        //         longitude: 2.2,
        //         category: 1,
        //         description: "test".into(),
        //         discoverer: 2,
        //     },
        // )
        // .unwrap();
        let (l, total) = find(
            &conn,
            Query {
                name: None,
                latitude: 1.1,
                longitude: 2.11111119,
                radius: 10000.0,
                category: None,
                limit: 10,
                offset: 0,
            },
        )
        .unwrap();
        assert!(total == 2);
        assert!(l.len() == 2);
        assert!(l[0].0.name == "test");
        println!("location: {:?}, distance: {}", l[0].0, l[0].2);
    }
}
