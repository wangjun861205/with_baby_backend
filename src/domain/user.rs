use super::error::Error;

pub struct Insert {
    name: String,
    phone: String,
    password: String,
    salt: String,
}

pub struct Update {
    name: String,
    phone: String,
    password: String,
    salt: String,
}

pub struct Query {
    name: String,
    phone: String,
}

pub struct User {
    id: i32,
    name: String,
    phone: String,
    password: String,
    salt: String,
    create_on: String,
    update_on: String,
}

pub trait UserPersister {
    fn insert_user(user: Insert) -> Result<i32, Error>;
    fn update_user(id: i32, user: Update) -> Result<i32, Error>;
    fn delete_user(id: i32) -> Result<i32, Error>;
    fn query_user(query: Query) -> Result<(Vec<User>, i64), Error>;
    fn get_user(id: i32) -> Result<User, Error>;
}
