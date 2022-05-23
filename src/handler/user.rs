use super::Error;
use crate::domain::user;
use actix_web::web::Json;

fn register<UP, SG, PH>(
    persister: UP,
    salt_generator: SG,
    password_hasher: PH,
    Json(req): Json<user::Registration>,
) -> Result<Json<i32>, Error>
where
    UP: user::UserPersister,
    SG: user::SaltGenerator,
    PH: user::PasswordHasher,
{
    let token = user::register(persister, salt_generator, password_hasher, req)?;
    Ok(Json(token))
}
