use crate::schema::*;
use anyhow::{Context, Error};
use diesel::{insert_into, pg::Pg, Connection, Insertable, RunQueryDsl};

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
