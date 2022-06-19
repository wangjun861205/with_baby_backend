use crate::models::{Memory, MemoryCommand, MemoryQuery, MemoryUploadRel, Upload};
use crate::schema::*;
use anyhow::{Context, Error};
use diesel::{self, insert_into, pg::Pg, BelongingToDsl, Connection, ExpressionMethods, GroupedBy, QueryDsl, RunQueryDsl, TextExpressionMethods};

pub fn find<T>(conn: &T, query: MemoryQuery) -> Result<(Vec<(Memory, Vec<Upload>)>, i64), Error>
where
    T: Connection<Backend = Pg>,
{
    let mut q = memories::table.limit(query.limit).offset(query.offset).into_boxed();
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
    let total = c.count().get_result(conn).context("failed to find memory")?;
    let ms: Vec<Memory> = q.load(conn)?;
    let images: Vec<Vec<Upload>> = MemoryUploadRel::belonging_to(&ms)
        .inner_join(uploads::table)
        .select((memory_upload_rels::all_columns, uploads::all_columns))
        .load::<(MemoryUploadRel, Upload)>(conn)?
        .grouped_by(&ms)
        .into_iter()
        .map(|l| l.into_iter().map(|(_, u)| u).collect())
        .collect();
    Ok((ms.into_iter().zip(images).collect(), total))
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
