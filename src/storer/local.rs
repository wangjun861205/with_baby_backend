use crate::domain::upload::UploadStorer;
use anyhow::Error;
use async_trait::async_trait;
use bytes::Bytes;
use futures::{Sink, SinkExt, Stream, StreamExt};
use std::os::unix::prelude::FileExt;
use std::pin::Pin;
use std::{
    fs::File,
    io::{Read, Write},
    task::{Context, Poll},
};

use infer::get_from_path;
use std::path;
use uuid::Uuid;

pub struct AsyncFile {
    file: File,
}

impl AsyncFile {
    pub fn new(file: File) -> Self {
        Self { file }
    }
}

impl Sink<Bytes> for AsyncFile {
    type Error = Error;
    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(mut self: Pin<&mut Self>, item: Bytes) -> Result<(), Self::Error> {
        self.file.write_all(&item)?;
        Ok(())
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(self.file.flush().map_err(|e| Error::from(e)))
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

impl Stream for AsyncFile {
    type Item = Result<Bytes, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut buffer = vec![0u8; 1024];
        match self.file.read(&mut buffer) {
            Err(e) => return Poll::Ready(Some(Err(Error::from(e)))),
            Ok(n) => {
                if n == 0 {
                    return Poll::Ready(None);
                }
                buffer.resize(n, 0);
                return Poll::Ready(Some(Ok(buffer.into())));
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct LocalStore;

impl LocalStore {
    pub fn new() -> Self {
        Self {}
    }
}
impl UploadStorer<AsyncFile, AsyncFile> for LocalStore {
    fn store(&self) -> Result<(AsyncFile, String), Error> {
        let name = Uuid::new_v4().to_string();
        let file = File::create(format!("/tmp/{}", name))?;
        Ok((AsyncFile::new(file), name))
    }

    fn get(&self, fetch_code: &str) -> Result<AsyncFile, Error> {
        let file = File::open(format!("/tmp/{}", fetch_code))?;
        Ok(AsyncFile::new(file))
    }

    fn mime(&self, fetch_code: &str) -> Result<String, Error> {
        let mime = get_from_path(path::Path::new("/tmp").join(fetch_code))?;
        if let Some(t) = mime {
            return Ok(t.to_string());
        }
        Ok("".into())
    }
}
