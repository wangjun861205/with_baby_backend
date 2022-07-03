use crate::models::{Comment, CommentInsert, CommentUpdate};
use crate::schema::comments;
use diesel::{insert_into, pg::Pg, result::Error, select, BoolExpressionMethods, Connection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use std::default::Default;

#[derive(Debug, Default)]
pub struct Query {
    pub rank_gt: Option<i32>,
    pub rank_lt: Option<i32>,
    pub location: Option<i32>,
    pub limit: i64,
    pub offset: i64,
}

pub fn query<T>(conn: &T, query: Query) -> Result<(Vec<Comment>, i64), Error>
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
    let total = c.count().get_result(conn)?;
    let list = q.order(comments::update_on.desc()).load(conn)?;
    Ok((list, total))
}

pub fn insert<T>(conn: &T, ins: CommentInsert) -> Result<i32, Error>
where
    T: Connection<Backend = Pg>,
{
    insert_into(comments::table).values(ins).returning(comments::id).get_result(conn)
}

pub fn update<T>(conn: &T, user: i32, loc: i32, upd: CommentUpdate) -> Result<usize, Error>
where
    T: Connection<Backend = Pg>,
{
    diesel::update(comments::table.filter(comments::user.eq(user).and(comments::location.eq(loc)))).set(upd).execute(conn)
}

pub fn may_get<T>(conn: &T, user: i32, loc: i32) -> Result<Option<Comment>, Error>
where
    T: Connection<Backend = Pg>,
{
    comments::table.filter(comments::user.eq(user).and(comments::location.eq(loc))).get_result(conn).optional()
}

pub fn may_get_for_update<T>(conn: &T, user: i32, loc: i32) -> Result<Option<Comment>, Error>
where
    T: Connection<Backend = Pg>,
{
    comments::table.filter(comments::user.eq(user).and(comments::location.eq(loc))).for_update().get_result(conn).optional()
}
