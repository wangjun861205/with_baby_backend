use crate::models::{Location, Memory, MemoryCommand, MemoryOrderBy, MemoryQuery, MemoryUploadRel, Upload};
use crate::schema::*;
use anyhow::{Context, Error};
use diesel::{self, delete, dsl::sql, insert_into, pg::Pg, sql_types::Double, BelongingToDsl, Connection, ExpressionMethods, GroupedBy, QueryDsl, RunQueryDsl, TextExpressionMethods};
use itertools::{multiunzip, unfold};

pub fn find<T>(conn: &T, query: MemoryQuery) -> Result<(Vec<Memory>, Vec<Location>, Vec<f64>, i64), Error>
where
    T: Connection<Backend = Pg>,
{
    let mut q = memories::table
        .inner_join(locations::table)
        .select((
            memories::all_columns,
            locations::all_columns,
            sql::<Double>(&format!(
                "earth_distance(ll_to_earth({}, {}), ll_to_earth(locations.latitude, locations.longitude)) AS distance",
                query.latitude, query.longitude
            )),
        ))
        .limit(query.limit)
        .offset(query.offset)
        .into_boxed();
    let mut c = memories::table.into_boxed();
    if let Some(title) = query.title {
        q = q.filter(memories::title.like(format!("%{}%", title)));
        c = c.filter(memories::title.like(format!("%{}%", title)));
    }
    if let Some(owner) = query.owner {
        q = q.filter(memories::owner.eq(owner));
        c = c.filter(memories::owner.eq(owner));
    }
    if let Some(location) = query.location {
        q = q.filter(memories::location.eq(location));
        c = c.filter(memories::location.eq(location));
    }
    if let Some(create_before) = query.create_before {
        q = q.filter(memories::create_on.lt(create_before));
        c = c.filter(memories::create_on.lt(create_before));
    }
    if let Some(create_after) = query.create_after {
        q = q.filter(memories::create_on.lt(create_after));
        c = c.filter(memories::create_on.lt(create_after));
    }
    match query.order_by {
        MemoryOrderBy::CreateOn => q = q.order_by(memories::create_on),
        MemoryOrderBy::CreateOnDesc => q = q.order_by(memories::create_on.desc()),
        MemoryOrderBy::UpdateOn => q = q.order_by(memories::update_on),
        MemoryOrderBy::UpdateOnDesc => q = q.order_by(memories::update_on.desc()),
        MemoryOrderBy::Distance => q = q.order_by(sql::<Double>("distance")),
        MemoryOrderBy::DistanceDesc => q = q.order_by(sql::<Double>("distance desc")),
        MemoryOrderBy::Title => q = q.order_by(memories::title),
        MemoryOrderBy::TitleDesc => q = q.order_by(memories::title.desc()),
    }
    let total = c.count().get_result(conn).context("failed to find memory")?;
    let list: Vec<(Memory, Location, f64)> = q.load::<(Memory, Location, f64)>(conn)?;
    let (m, l, d) = multiunzip(list);
    Ok((m, l, d, total))
}

pub fn insert<T>(conn: &T, ins: MemoryCommand) -> Result<i32, Error>
where
    T: Connection<Backend = Pg>,
{
    insert_into(memories::table).values(ins).returning(memories::id).get_result(conn).context("failed to insert memory")
}

pub fn update<T>(conn: &T, id: i32, upd: MemoryCommand) -> Result<usize, Error>
where
    T: Connection<Backend = Pg>,
{
    diesel::update(memories::table).filter(memories::id.eq(id)).set(upd).execute(conn).context("failed to update memory")
}

pub fn clear_images<T>(conn: &T, id: i32) -> Result<usize, Error>
where
    T: Connection<Backend = Pg>,
{
    delete(memory_upload_rels::table)
        .filter(memory_upload_rels::memory.eq(id))
        .execute(conn)
        .context("failed to clear images of memory")
}

pub fn add_images<T>(conn: &T, id: i32, images: Vec<i32>) -> Result<usize, Error>
where
    T: Connection<Backend = Pg>,
{
    insert_into(memory_upload_rels::table)
        .values(
            images
                .into_iter()
                .map(|img| (memory_upload_rels::memory.eq(id), memory_upload_rels::upload.eq(img)))
                .collect::<Vec<_>>(),
        )
        .execute(conn)
        .context("failed to add images for memory")
}

pub fn query_images<T>(conn: &T, memories: &Vec<Memory>) -> Result<Vec<Vec<Upload>>, Error>
where
    T: Connection<Backend = Pg>,
{
    let upload_set: Vec<Vec<(MemoryUploadRel, Upload)>> = MemoryUploadRel::belonging_to(memories).inner_join(uploads::table).load(conn)?.grouped_by(memories);
    Ok(upload_set.into_iter().map(|upls| upls.into_iter().map(|(_, u)| u).collect()).collect())
}
