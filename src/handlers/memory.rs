use super::{Error, PgPool, QueryResponse};
use crate::dao::memory::{add_images, find, insert, query_images};
use crate::models::{Memory, MemoryCommand, MemoryQuery, Upload};
use crate::token::UID;
use actix_web::{
    web::{get, post, Data, Json, Path, Query},
    Scope,
};
use diesel::Connection;
use serde::Deserialize;

pub fn register(scope: Scope) -> Scope {
    scope.route("/{id}/memories", post().to(create)).route("/{id}/memories", get().to(list))
}

#[derive(Debug, Deserialize)]
pub struct CreateBody {
    pub title: String,
    pub content: String,
    pub images: Vec<i32>,
}
pub async fn create(pool: Data<PgPool>, uid: UID, location: Path<(i32,)>, Json(body): Json<CreateBody>) -> Result<Json<i32>, Error> {
    let conn = pool.get()?;
    let id = conn.transaction::<i32, anyhow::Error, _>(|| {
        let id = insert(
            &conn,
            MemoryCommand {
                title: body.title,
                content: body.content,
                owner: uid.0,
                location: location.0,
            },
        )?;
        add_images(&conn, id, body.images)?;
        Ok(id)
    })?;
    Ok(Json(id))
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    limit: i64,
    offset: i64,
}

pub async fn list(pool: Data<PgPool>, location: Path<(i32,)>, Query(ListParams { limit, offset }): Query<ListParams>) -> Result<Json<QueryResponse<(Memory, Vec<Upload>)>>, Error> {
    let conn = pool.get()?;
    let (list, total) = conn.transaction::<(Vec<(Memory, Vec<Upload>)>, i64), anyhow::Error, _>(|| {
        let (mems, total) = find(
            &conn,
            MemoryQuery {
                title: None,
                owner: None,
                location: Some(location.0),
                create_before: None,
                create_after: None,
                limit: limit,
                offset: offset,
            },
        )?;
        let imgs = query_images(&conn, &mems)?;
        Ok((mems.into_iter().zip(imgs).map(|(m, i)| (m, i)).collect(), total))
    })?;
    Ok(Json(QueryResponse::new(list, total)))
}
