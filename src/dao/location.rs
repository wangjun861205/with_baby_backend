use crate::domain::upload;
use crate::models::{Location, LocationInsertion, LocationUploadRel, Upload, User};
use crate::schema::*;
use crate::serde::{Deserialize, Serialize};
use anyhow::{Context, Error};
use diesel::{
    dsl::{self, sql},
    insert_into,
    pg::{Pg, PgConnection},
    sql_types::Double,
    BelongingToDsl, Connection, ExpressionMethods, GroupedBy, QueryDsl, RunQueryDsl, TextExpressionMethods,
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Query {
    pub name: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub radius: f64,
    pub category: Option<i32>,
    pub limit: i64,
    pub offset: i64,
}

pub fn find<T>(conn: &T, query: Query) -> Result<(Vec<(Location, User, f64, Vec<Upload>)>, i64), Error>
where
    T: Connection<Backend = Pg>,
{
    let mut c = locations::table
        .inner_join(users::table)
        .filter(sql(&format!(
            "earth_box(ll_to_earth({}, {}), {}) @> ll_to_earth(locations.latitude, locations.longitude)",
            query.latitude, query.longitude, query.radius
        )))
        .into_boxed();
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
        .filter(sql(&format!(
            "earth_box(ll_to_earth({}, {}), {}) @> ll_to_earth(locations.latitude, locations.longitude)",
            query.latitude, query.longitude, query.radius
        )))
        .limit(query.limit)
        .offset(query.offset)
        .into_boxed();

    if let Some(name) = query.name {
        c = c.filter(locations::name.like(format!("%{}%", name)));
        q = q.filter(locations::name.like(format!("%{}%", name)));
    }
    if let Some(category) = query.category {
        c = c.filter(locations::category.eq(category));
        q = q.filter(locations::category.eq(category));
    }
    q = q.order_by(sql::<Double>("distance"));
    let total = c.count().get_result(conn).context("failed to find locations")?;
    let l: Vec<(Location, User, f64)> = q.load(conn).context("failed to find locations")?;
    let locs: Vec<Location> = l.iter().map(|(l, _, _)| l.clone()).collect();
    let images: Vec<Vec<Upload>> = LocationUploadRel::belonging_to(&locs)
        .inner_join(uploads::table)
        .load::<(LocationUploadRel, Upload)>(conn)?
        .grouped_by(&locs)
        .into_iter()
        .map(|l| l.into_iter().map(|(_, u)| u).collect())
        .collect();
    Ok((l.into_iter().zip(images).map(|((l, u, d), imgs)| (l, u, d, imgs)).collect(), total))
}

pub fn get<T>(conn: &T, id: i32, latitude: f64, longitude: f64) -> Result<(Location, User, f64), Error>
where
    T: Connection<Backend = Pg>,
{
    locations::table
        .inner_join(users::table)
        .select((
            locations::all_columns,
            users::all_columns,
            sql::<Double>(&format!("earth_distance(ll_to_earth({}, {}), ll_to_earth(latitude, longitude))", latitude, longitude)),
        ))
        .filter(locations::id.eq(id))
        .get_result(conn)
        .context("failed to get location")
}

pub fn insert(conn: &PgConnection, loc: LocationInsertion) -> Result<i32, Error> {
    insert_into(locations::table).values(loc).returning(locations::id).get_result(conn).context("failed to insert location")
}

pub fn update(conn: &PgConnection, loc: Location) -> Result<usize, Error> {
    diesel::update(locations::table)
        .filter(locations::id.eq(loc.id))
        .set(loc)
        .execute(conn)
        .context("failed to update location")
}

pub fn exists<T>(conn: &T, query: Query) -> Result<bool, Error>
where
    T: Connection<Backend = Pg>,
{
    let mut q = locations::table
        .filter(sql(&format!(
            "earth_box(ll_to_earth({}, {}), {}) @> ll_to_earth(latitude, longitude)",
            query.latitude, query.longitude, query.radius
        )))
        .into_boxed();
    if let Some(name) = query.name {
        q = q.filter(locations::name.like(format!("%{}%", name)))
    }
    if let Some(category) = query.category {
        q = q.filter(locations::category.eq(category))
    }
    dsl::select(dsl::exists(q)).get_result(conn).context("failed to check location exists")
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
