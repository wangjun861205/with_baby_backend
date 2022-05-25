use hex::encode;
use sha2::Digest;

use crate::domain::user::PasswordHasher;

pub struct Hasher;

impl Hasher {
    pub fn new() -> Self {
        Self {}
    }
}

impl PasswordHasher for Hasher {
    fn hash(&self, salt: &str, password: &str) -> String {
        let mut h = sha2::Sha256::new();
        h.update(salt.as_bytes());
        h.update(password.as_bytes());
        let res = h.finalize();
        encode(res)
    }
}
