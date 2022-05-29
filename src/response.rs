use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ListResponse<T> {
    list: Vec<T>,
    total: i64,
}

impl<T> ListResponse<T> {
    pub fn new(list: Vec<T>, total: i64) -> Self {
        Self { list, total }
    }
}
