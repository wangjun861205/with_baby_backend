use crate::dao::{comment, rank_aggregation};
use crate::error::Error;
use crate::handlers::PgPool;
use crate::models::{Comment, CommentInsert, CommentUpdate, RankAggregationUpdate};
use crate::response::ListResponse;
use crate::token::UID;
use actix_web::{
    http::StatusCode,
    web::{get, post, put, scope, Data, Json, Path, Query},
    HttpResponse, Scope,
};
use diesel::Connection;
use serde::Deserialize;
use std::default::Default;

pub(crate) fn register(scope: Scope) -> Scope {
    scope
        .route("/{loc}/comments", get().to(comments_of_location))
        .route("/{loc}/comments", post().to(create))
        .route("/{loc}/comments", put().to(upsert))
        .route("/{loc}/comment", get().to(may_get))
}

#[derive(Debug, Deserialize)]
pub struct Create {
    content: String,
    rank: i32,
}

pub async fn may_get(pool: Data<PgPool>, UID(uid): UID, loc: Path<(i32,)>) -> Result<Json<Option<Comment>>, Error> {
    Ok(Json(comment::may_get(&pool.get()?, uid, loc.0)?))
}

pub async fn create(pool: Data<PgPool>, UID(uid): UID, loc: Path<(i32,)>, Json(body): Json<Create>) -> Result<HttpResponse, Error> {
    match comment::insert(
        &pool.get()?,
        CommentInsert {
            rank: body.rank,
            content: body.content,
            user: uid,
            location: loc.0,
        },
    ) {
        Ok(id) => return Ok(HttpResponse::build(StatusCode::OK).json(id)),
        Err(e) => match e {
            diesel::result::Error::DatabaseError(kind, _) => match kind {
                diesel::result::DatabaseErrorKind::UniqueViolation => {
                    return Ok(HttpResponse::build(StatusCode::CONFLICT).finish());
                }
                _ => return Err(Error::from(e)),
            },
            _ => return Err(Error::from(e)),
        },
    }
}

pub async fn upsert(pool: Data<PgPool>, UID(uid): UID, loc: Path<(i32,)>, Json(body): Json<CommentUpdate>) -> Result<Json<usize>, Error> {
    let conn = pool.get()?;
    let res = conn.transaction::<usize, Error, _>(|| {
        let agg = rank_aggregation::get_for_update(&conn, loc.0)?;
        let mut ori_rank = 0;
        let new_rank = body.rank;
        let mut res = 0usize;
        let mut added_count = 0;
        if let Some(cmt) = comment::may_get_for_update(&conn, uid, loc.0)? {
            res = comment::update(&conn, uid, loc.0, body)?;
            ori_rank = cmt.rank;
        } else {
            comment::insert(
                &conn,
                CommentInsert {
                    rank: body.rank,
                    content: body.content,
                    user: uid,
                    location: loc.0,
                },
            )?;
            added_count = 1;
        }
        rank_aggregation::update(
            &conn,
            loc.0,
            RankAggregationUpdate {
                total: agg.total + new_rank as i64 - ori_rank as i64,
                count: agg.count + added_count,
            },
        )?;
        Ok(res)
    })?;
    Ok(Json(res))
}

#[derive(Debug, Deserialize)]
pub struct CommentsOfLocation {
    limit: i64,
    offset: i64,
}

pub(crate) async fn comments_of_location(pool: Data<PgPool>, loc: Path<(i32,)>, Query(CommentsOfLocation { limit, offset }): Query<CommentsOfLocation>) -> Result<Json<ListResponse<Comment>>, Error> {
    let (list, total) = comment::query(
        &pool.get()?,
        comment::Query {
            limit: limit,
            offset: offset,
            location: Some(loc.0),
            ..Default::default()
        },
    )?;
    Ok(Json(ListResponse::new(list, total)))
}
