use crate::models::{Comment, CommentCommand};
use crate::schema::comments;
use anyhow::{Context, Error};
use diesel::{insert_into, pg::Pg, BoolExpressionMethods, Connection, ExpressionMethods, QueryDsl, RunQueryDsl};

pub struct Query {
    rank_gt: Option<i32>,
    rank_lt: Option<i32>,
    location: Option<i32>,
    limit: i64,
    offset: i64,
}

pub fn find<T>(conn: &T, query: Query) -> Result<(Vec<Comment>, i64), Error>
where
    T: Connection<Backend = Pg>,
{
    let mut q = comments::table.limit(query.limit).offset(query.offset).into_boxed();
    let mut c = comments::table.into_boxed();
    if let Some(rank_gt) = query.rank_gt {
        q = q.filter(comments::rank.gt(rank_gt));
        c = c.filter(comments::rank.gt(rank_gt));
    }
    if let Some(rank_lt) = query.rank_lt {
        q = q.filter(comments::rank.lt(rank_lt));
        c = c.filter(comments::rank.lt(rank_lt));
    }
    if let Some(location) = query.location {
        q = q.filter(comments::location.eq(location));
        c = c.filter(comments::location.eq(location));
    }
    let total = c.count().get_result(conn).context("failed to find comments")?;
    let list = q.order(comments::update_on.desc()).load(conn).context("failed to find comments")?;
    Ok((list, total))
}

pub fn insert<T>(conn: &T, ins: CommentCommand) -> Result<i32, Error>
where
    T: Connection<Backend = Pg>,
{
    insert_into(comments::table)
        .values(ins)
        .returning(comments::id)
        .get_result(conn)
        .context("failed to insert into comments")
}

pub fn update<T>(conn: &T, id: i32, upd: CommentCommand) -> Result<usize, Error>
where
    T: Connection<Backend = Pg>,
{
    diesel::update(comments::table).filter(comments::id.eq(id)).set(upd).execute(conn).context("failed to update comment")
}
