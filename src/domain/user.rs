use anyhow::Error;
use serde::{Deserialize, Serialize};

pub struct Insert {
    pub name: String,
    pub phone: String,
    pub password: String,
    pub salt: String,
}

pub struct Update {
    pub name: String,
    pub phone: String,
    pub password: String,
    pub salt: String,
}

pub struct Query {
    pub name: Option<String>,
    pub phone: Option<String>,
}

pub struct User {
    id: i32,
    name: String,
    phone: String,
    password: String,
    salt: String,
    create_on: chrono::NaiveDateTime,
    update_on: chrono::NaiveDateTime,
}

pub trait UserPersister {
    fn insert_user(&mut self, user: Insert) -> Result<i32, Error>;
    fn update_user(&mut self, id: i32, user: Update) -> Result<usize, Error>;
    fn delete_user(&mut self, id: i32) -> Result<usize, Error>;
    fn query_user(&mut self, query: Query) -> Result<(Vec<User>, i64), Error>;
    fn get_user(&mut self, id: i32) -> Result<User, Error>;
    fn get_user_by_phone(&self, phone: &str) -> Result<User, Error>;
}

#[derive(Debug, Serialize)]
pub struct Registration {
    pub name: String,
    pub phone: String,
    pub password: String,
}

pub struct Login {
    phone: String,
    password: String,
}

pub trait SaltGenerator {
    fn gen(&self) -> String;
}

pub trait PasswordHasher {
    fn hash(&self, salt: &str, password: &str) -> String;
}

pub fn register<P, SG, PH>(
    mut persister: P,
    salt_generator: SG,
    password_hasher: PH,
    req: Registration,
) -> Result<i32, Error>
where
    P: UserPersister,
    SG: SaltGenerator,
    PH: PasswordHasher,
{
    let salt = salt_generator.gen();
    let hashed_password = password_hasher.hash(&salt, &req.password);
    let id = persister.insert_user(Insert {
        name: req.name,
        phone: req.phone,
        password: hashed_password,
        salt: salt,
    })?;
    Ok(id)
}

pub trait TokenGenerator {
    fn gen(&self, id: i32) -> String;
}

pub fn login<P, RF, PH, TG>(
    persister: P,
    password_hasher: PH,
    token_generator: TG,
    req: Login,
) -> Result<String, Error>
where
    P: UserPersister,
    PH: PasswordHasher,
    TG: TokenGenerator,
{
    let user = persister.get_user_by_phone(&req.phone)?;
    let hashed_password = password_hasher.hash(&user.salt, &req.password);
    if hashed_password != user.password {
        return Err(Error::msg("invalid phone or password"));
    }
    Ok(token_generator.gen(user.id))
}
