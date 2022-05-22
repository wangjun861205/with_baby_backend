use super::error::Error;
use hex;
use rand::{thread_rng, Rng};
use sha2::{Digest, Sha256};

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
    fn insert_user(&mut self, user: Insert) -> Result<i32, Error>;
    fn update_user(&mut self, id: i32, user: Update) -> Result<i32, Error>;
    fn delete_user(&mut self, id: i32) -> Result<i32, Error>;
    fn query_user(&mut self, query: Query) -> Result<(Vec<User>, i64), Error>;
    fn get_user(&mut self, id: i32) -> Result<User, Error>;
    fn get_user_by_phone(&self, phone: &str) -> Result<User, Error>;
}

pub struct Registration {
    name: String,
    phone: String,
    password: String,
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

pub fn register<P, RF, SG, PH>(
    mut persister: P,
    request_func: RF,
    salt_generator: SG,
    password_hasher: PH,
) -> Result<i32, Error>
where
    P: UserPersister,
    RF: Fn() -> Registration,
    SG: SaltGenerator,
    PH: PasswordHasher,
{
    let reg = request_func();
    let salt = salt_generator.gen();
    let hashed_password = password_hasher.hash(&salt, &reg.password);
    let id = persister.insert_user(Insert {
        name: reg.name,
        phone: reg.phone,
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
    request_func: RF,
    password_hasher: PH,
    token_generator: TG,
) -> Result<String, Error>
where
    P: UserPersister,
    RF: Fn() -> Login,
    PH: PasswordHasher,
    TG: TokenGenerator,
{
    let req = request_func();
    let user = persister.get_user_by_phone(&req.phone)?;
    let hashed_password = password_hasher.hash(&user.salt, &req.password);
    if hashed_password != user.password {
        return Err(Error::new("invalid phone or password"));
    }
    Ok(token_generator.gen(user.id))
}
