use crate::models::Upload;
use crate::schema::*;
use anyhow::{Context, Error};
use diesel::{insert_into, pg::Pg, Connection, ExpressionMethods, Insertable, QueryDsl, RunQueryDsl};

#[derive(Debug, Insertable)]
#[table_name = "location_upload_rels"]
pub struct LocationUploadRelInsertion {
    pub location_id: i32,
    pub upload_id: i32,
}

pub fn insert_location_upload_rel<T>(conn: &T, ins: LocationUploadRelInsertion) -> Result<i32, Error>
where
    T: Connection<Backend = Pg>,
{
    insert_into(location_upload_rels::table)
        .values(ins)
        .returning(location_upload_rels::id)
        .get_result(conn)
        .context("failed to insert location upload relation")
}

#[derive(Debug, Default)]
pub struct Query {
    pub location: Option<i32>,
    pub fetch_code: Option<String>,
    pub limit: i64,
    pub offset: i64,
}

pub fn find<T>(conn: &T, query: Query) -> Result<(Vec<Upload>, i64), Error>
where
    T: Connection<Backend = Pg>,
{
    if query.location.is_none() && query.fetch_code.is_none() {
        return Err(Error::msg("invalid query").context("failed to find uploads"));
    }
    let mut q = uploads::table
        .inner_join(location_upload_rels::table)
        .select(uploads::all_columns)
        .limit(query.limit)
        .offset(query.offset)
        .into_boxed();
    let mut c = uploads::table.inner_join(location_upload_rels::table).into_boxed();
    if let Some(location) = query.location {
        q = q.filter(location_upload_rels::location_id.eq(location));
        c = c.filter(location_upload_rels::location_id.eq(location));
    }
    if let Some(fetch_code) = query.fetch_code {
        q = q.filter(uploads::fetch_code.eq(fetch_code.clone()));
        c = c.filter(uploads::fetch_code.eq(fetch_code.clone()));
    }
    let total = c.count().get_result(conn).context("failed to find uploads")?;
    let list = q.load(conn).context("failed to find uploads")?;
    Ok((list, total))
}
