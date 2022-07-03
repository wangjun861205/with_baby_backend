use crate::models::{Location, LocationInsertion, LocationUpdating, LocationUploadRel, Upload, User};
use crate::schema::*;
use crate::serde::Deserialize;
use anyhow::{Context, Error};
use diesel::{
    delete,
    dsl::{self, sql},
    insert_into,
    pg::{Pg, PgConnection},
    sql_types::Double,
    BelongingToDsl, Connection, ExpressionMethods, QueryDsl, RunQueryDsl, TextExpressionMethods,
};

#[derive(Debug, Default, Deserialize)]
pub struct Query {
    pub name: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub radius: f64,
    pub category: Option<i32>,
    pub limit: i64,
    pub offset: i64,
    pub order_by: OrderBy,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderBy {
    DistanceAsc,
    DistanceDesc,
    CreateOnAsc,
    CreateOnDesc,
    UpdateOnAsc,
    UpdateOnDesc,
    NameAsc,
    NameDesc,
}

impl Default for OrderBy {
    fn default() -> Self {
        Self::DistanceAsc
    }
}

pub fn query<T>(conn: &T, query: Query) -> Result<((Vec<Location>, Vec<f64>), i64), Error>
where
    T: Connection<Backend = Pg>,
{
    let mut c = locations::table
        .filter(sql(&format!(
            "earth_box(ll_to_earth({}, {}), {}) @> ll_to_earth(locations.latitude, locations.longitude)",
            query.latitude, query.longitude, query.radius
        )))
        .into_boxed();
    let mut q = locations::table
        .select((
            locations::all_columns,
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
    match query.order_by {
        OrderBy::DistanceAsc => q = q.order_by(sql::<Double>("distance")),
        OrderBy::CreateOnAsc => q = q.order_by(locations::create_on),
        OrderBy::UpdateOnAsc => q = q.order_by(locations::update_on),
        OrderBy::NameAsc => q = q.order_by(locations::name),
        OrderBy::DistanceDesc => q = q.order_by(sql::<Double>("distance DESC")),
        OrderBy::CreateOnDesc => q = q.order_by(locations::create_on.desc()),
        OrderBy::UpdateOnDesc => q = q.order_by(locations::update_on.desc()),
        OrderBy::NameDesc => q = q.order_by(locations::name.desc()),
    }
    let total = c.count().get_result(conn).context("failed to find locations")?;
    let rows: Vec<(Location, f64)> = q.load(conn).context("failed to find locations")?;
    let (locs, dists) = rows.into_iter().unzip();
    Ok(((locs, dists), total))
}

pub fn get<T>(conn: &T, id: i32, latitude: f64, longitude: f64) -> Result<(Location, f64), Error>
where
    T: Connection<Backend = Pg>,
{
    let (loc, dist) = locations::table
        .inner_join(users::table)
        .select((
            locations::all_columns,
            sql::<Double>(&format!("earth_distance(ll_to_earth({}, {}), ll_to_earth(latitude, longitude))", latitude, longitude)),
        ))
        .filter(locations::id.eq(id))
        .get_result::<(Location, f64)>(conn)
        .context("failed to get location")?;
    Ok((loc, dist))
}

pub fn get_without_coord<T>(conn: &T, id: i32) -> Result<(Location, User, Vec<Upload>), Error>
where
    T: Connection<Backend = Pg>,
{
    let (loc, user) = locations::table
        .inner_join(users::table)
        .select((locations::all_columns, users::all_columns))
        .filter(locations::id.eq(id))
        .get_result::<(Location, User)>(conn)
        .context("failed to get location")?;
    let images = LocationUploadRel::belonging_to(&loc)
        .inner_join(uploads::table)
        .select(uploads::all_columns)
        .load(conn)
        .context("failed to get location")?;
    Ok((loc, user, images))
}

pub fn insert(conn: &PgConnection, loc: LocationInsertion) -> Result<i32, Error> {
    insert_into(locations::table).values(loc).returning(locations::id).get_result(conn).context("failed to insert location")
}

pub fn update(conn: &PgConnection, id: i32, loc: LocationUpdating) -> Result<usize, Error> {
    diesel::update(locations::table)
        .filter(locations::id.eq(id))
        .set(loc)
        .execute(conn)
        .context("failed to update location")
}

pub fn clear_images(conn: &PgConnection, id: i32) -> Result<usize, Error> {
    delete(location_upload_rels::table)
        .filter(location_upload_rels::location_id.eq(id))
        .execute(conn)
        .context("failed to clear images of loacation")
}

pub fn add_images(conn: &PgConnection, id: i32, images: Vec<i32>) -> Result<usize, Error> {
    insert_into(location_upload_rels::table)
        .values(
            images
                .into_iter()
                .map(|img_id| (location_upload_rels::location_id.eq(id), location_upload_rels::upload_id.eq(img_id)))
                .collect::<Vec<_>>(),
        )
        .execute(conn)
        .context("failed to add images for location")
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
