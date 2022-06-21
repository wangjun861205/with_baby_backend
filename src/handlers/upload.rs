use super::{Error, PgPool};
use crate::domain::upload;
use crate::persister::postgres::PostgresPersister;
use crate::storer::local::{AsyncFile, LocalStore};
use crate::token::UID;
use actix_multipart::Multipart;
use actix_web::{
    body::BodyStream,
    http::StatusCode,
    web::{get, post, route, Data, Json, Path},
    HttpResponse, HttpResponseBuilder, Resource, Scope,
};
use anyhow::Context;
use futures::{Stream, StreamExt};
use std::fs::File;
use std::io::Write;

pub fn register_route(scope: &str) -> Scope {
    Scope::new(scope).route("", post().to(upload)).route("/{id}", get().to(fetch))
}

pub async fn upload(uid: UID, mut multi: Multipart, pool: Data<PgPool>) -> Result<Json<Vec<i32>>, Error> {
    let storer = LocalStore::new();
    let mut ids = Vec::new();
    while let Some(Ok(field)) = multi.next().await {
        let persister = PostgresPersister::new(pool.get().map_err(|e| anyhow::Error::from(e))?);
        let f = field.map(|v| v.map_err(|e| anyhow::Error::from(e)));
        ids.push(upload::upload(f, storer.clone(), persister, uid.0).await?);
    }
    Ok(Json(ids))
}

pub async fn fetch(id: Path<(i32,)>, pool: Data<PgPool>) -> Result<HttpResponse, Error> {
    let persister = PostgresPersister::new(pool.get().map_err(|e| anyhow::Error::from(e))?);
    let storer = LocalStore::new();
    let (stream, mime) = upload::get(id.0, persister, storer).await?;
    Ok(HttpResponse::Ok().content_type(mime).body(BodyStream::new(stream)))
}
