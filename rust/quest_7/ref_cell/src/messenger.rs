use std::cell::Cell;
use std::rc::Rc;
pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Tracker<'l, L: Logger> {
    logger: &'l L,
    value: Cell<usize>,
    max: usize,
}

impl<'l, L: Logger> Tracker<'l, L> {
    pub fn new(logger: &'l L, max: usize) -> Self {
        Self {logger: logger, value: Cell::new(0), max: max,
        }
    }

    pub fn set_value<T>(&self, rc: &Rc<T>) {
        self.value.set(Rc::strong_count(rc));
        let perc = self.value.get() * 100 / self.max;

        if perc >= 100 {
            self.logger.error("Error: you are over your quota!")
        } else {
            self.logger.warning(&format!("Warning: you have used up over {}% of your quota! Proceeds with precaution", perc))
        }
    }

    pub fn peek<T>(&self, rc: &Rc<T>) {
        let perc = Rc::strong_count(rc) * 100 / self.max;
        self.logger.info(&format!("Info: you are using up too {}% of your quote", perc))
    }
}
