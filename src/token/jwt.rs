use crate::handler::Tokener;

pub struct JWT {
    secret: String,
}

impl JWT {
    pub fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_owned(),
        }
    }
}
