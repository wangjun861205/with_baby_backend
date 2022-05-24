use super::Error;
use crate::domain::user;
use crate::persister::postgres::PostgresPersister;
use actix_web::web::{Data, Json};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};

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
    fn validate(&self, token: &str) -> Result<(), anyhow::Error> {
        self.as_ref().validate(token)
    }
}

pub async fn signup<SG, PH>(
    db: Data<Pool<ConnectionManager<PgConnection>>>,
    salt_generator: Data<SG>,
    password_hasher: Data<PH>,
    Json(req): Json<user::Registration>,
) -> Result<Json<i32>, Error>
where
    SG: user::SaltGenerator,
    PH: user::PasswordHasher,
{
    let p = PostgresPersister::new(db.get().unwrap());
    let token = user::register(p, salt_generator, password_hasher, req)?;
    Ok(Json(token))
}

pub async fn signup<PH, TK>(
    db: Data<Pool<ConnectionManager<PgConnection>>>,
    password_hasher: Data<PH>,
    tokener: Data<TK>,
    Json(req): Json<user::Login>,
) -> Result<String, Error>
where
    PH: user::PasswordHasher,
    TK: super::Tokener,
{
    let p = PostgresPersister::new(db.get().unwrap());
    user::login(p, password_hasher)
}
