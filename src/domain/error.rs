use std::fmt::write;

pub struct Error {
    message: String,
    caused_by: Option<Box<Error>>,
}

impl Error {
    pub fn new(msg: &str) -> Self {
        Self {
            message: msg.to_owned(),
            caused_by: None,
        }
    }

    pub fn wrap(self, msg: &str) -> Self {
        Self {
            message: msg.to_owned(),
            caused_by: Some(Box::new(self)),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(reason) = &self.caused_by {
            return write(f, format_args!("{}: {}", self.message, reason));
        }
        write(f, format_args!("{}", self.message))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_chained_error() {
        let e1 = Error::new("level 1");
        let e2 = e1.wrap("level 2");
        let e3 = e2.wrap("level 3");
        println!("{}", e3);
    }
}
