use crate::domain::user::{self, LoginResult};
use crate::error::Error;
use crate::persister::postgres::PostgresPersister;
use actix_web::web::{Data, Json};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use serde::Serialize;

impl<T> user::SaltGenerator for Data<T>
where
    T: user::SaltGenerator,
{
    fn gen(&self) -> String {
        self.as_ref().gen()
    }
}

impl<T> user::PasswordHasher for Data<T>
where
    T: user::PasswordHasher,
{
    fn hash(&self, salt: &str, password: &str) -> String {
        self.as_ref().hash(salt, password)
    }
}

impl<T> super::Tokener for Data<T>
where
    T: super::Tokener,
{
    fn generate(&self, uid: i32) -> Result<String, anyhow::Error> {
        self.as_ref().generate(uid)
    }
    fn validate(&self, token: &str) -> Result<i32, anyhow::Error> {
        self.as_ref().validate(token)
    }
}

pub async fn signup<SG, PH>(db: Data<Pool<ConnectionManager<PgConnection>>>, salt_generator: Data<SG>, password_hasher: Data<PH>, Json(req): Json<user::Registration>) -> Result<Json<i32>, Error>
where
    SG: user::SaltGenerator,
    PH: user::PasswordHasher,
{
    let p = PostgresPersister::new(db.get().unwrap());
    let token = user::register(p, salt_generator, password_hasher, req)?;
    Ok(Json(token))
}

#[derive(Debug, Serialize)]
pub struct SigninResponse {
    id: i32,
    name: String,
    token: String,
    avatar: Option<i32>,
}

pub async fn signin<PH, TK>(db: Data<Pool<ConnectionManager<PgConnection>>>, password_hasher: Data<PH>, tokener: Data<TK>, Json(req): Json<user::Login>) -> Result<Json<SigninResponse>, Error>
where
    PH: user::PasswordHasher,
    TK: super::Tokener,
{
    let p = PostgresPersister::new(db.get().unwrap());
    let LoginResult { id, name, avatar } = user::login(p, password_hasher, req)?;
    let token = tokener.generate(id)?;
    Ok(Json(SigninResponse { id, name, token, avatar }))
}

#[derive(Debug, Serialize)]
pub struct User {
    id: i32,
    name: String,
}

impl From<user::User> for User {
    fn from(u: user::User) -> Self {
        Self { id: u.id, name: u.name }
    }
}
