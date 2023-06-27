use std::cell::{RefCell, Cell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pid: usize,
    command: String,
    workers: &'a Workers,
}


impl Workers {
    pub fn new() -> Self {
        Self {drops: Cell::new(0), states: RefCell::new(vec![])}
    }

    pub fn new_worker(&self, c: String) -> (usize, Thread) {
        self.states.borrow_mut().push(false);
        let pid = self.track_worker();
        (pid, Thread::new_thread(pid, c, self))
    }

    pub fn track_worker(&self) -> usize {
        self.states.borrow().len() - 1
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow()[id]
    }

    pub fn add_drop(&self, id: usize) {
        if self.is_dropped(id) {
            panic!("{} is already dropped", id)
        }
        self.states.borrow_mut()[id] = true;
        self.drops.set(self.drops.get() + 1);
    }
}

impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Self {
        Self {pid: p, command: c, workers: t,
        }
    }
    pub fn skill(self) {}
}

impl Drop for Thread<'_> {
    fn drop(&mut self) {
        self.workers.add_drop(self.pid)
    }
}