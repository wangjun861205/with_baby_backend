use crate::domain::eating::Eating;
use anyhow::{Context, Error};
use bytes::Bytes;
use chrono::NaiveDateTime;
use futures::{Sink, SinkExt, Stream, StreamExt};
use serde::Serialize;

static READ_BUFFER_SIZE: usize = 1024;
static WRITE_BUFFER_SIZE: usize = 1024;

pub trait UploadStorer<SM: Stream, SK: Sink<Bytes>> {
    fn store(&self) -> Result<(SK, String), Error>;
    fn get(&self, fetch_code: &str) -> Result<SM, Error>;
}

#[derive(Debug, Serialize)]
pub struct Upload {
    pub id: i32,
    pub fetch_code: String,
    pub owner: i32,
    pub create_on: NaiveDateTime,
    pub update_on: NaiveDateTime,
}

pub struct Insertion {
    pub fetch_code: String,
    pub owner: i32,
}

pub trait UploadPersister {
    fn insert_upload(&self, ins: Insertion) -> Result<i32, Error>;
    fn get_upload(&self, id: i32) -> Result<Upload, Error>;
    fn query_upload_by_ids(&self, ids: &Vec<i32>) -> Result<Vec<Upload>, Error>;
    fn query_upload_by_eatings(&self, eatings: &Vec<Eating>) -> Result<Vec<Vec<Upload>>, Error>;
}

pub async fn upload<S, P, SM, SK, IS>(mut stream: IS, storer: S, persister: P, uid: i32) -> Result<i32, Error>
where
    S: UploadStorer<SM, SK>,
    P: UploadPersister,
    SM: Stream,
    SK: Sink<Bytes, Error = Error> + Unpin,
    IS: Stream<Item = Result<Bytes, Error>> + Unpin,
{
    let (mut sink, fetch_code) = storer.store()?;
    for bs in stream.next().await {
        match bs {
            Ok(b) => {
                sink.send(b).await.context("failed to upload")?;
            }
            Err(e) => return Err(Error::from(e)),
        }
    }
    persister.insert_upload(Insertion { fetch_code: fetch_code, owner: uid }).context("failed to update")
}

pub async fn get<S, SM, P, SK>(id: i32, persister: P, storer: S) -> Result<SM, Error>
where
    SM: Stream,
    P: UploadPersister,
    S: UploadStorer<SM, SK>,
    SK: Sink<Bytes, Error = Error>,
{
    let u = persister.get_upload(id).context("failed to get uploaded file")?;
    storer.get(&u.fetch_code).context("failed to get uploaded file")
}
