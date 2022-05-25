use crate::domain::user::SaltGenerator;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Generator<T: Rng> {
    rng: Rc<RefCell<T>>,
}

impl<T: Rng> Generator<T> {
    pub fn new(rng: T) -> Self {
        Self {
            rng: Rc::new(RefCell::new(rng)),
        }
    }
}

impl<T: Rng> SaltGenerator for Generator<T> {
    fn gen(&self) -> String {
        let mut s = String::new();
        for _ in 0..64 {
            s.push(self.rng.borrow_mut().gen_range(33u8..127u8) as char);
        }
        s
    }
}
