use std::mem;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Worker {
    pub worker_type: String,
    pub worker_name: String,
    pub next_worker: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        Self{grade: None}
    }

    pub fn add_worker(&mut self, t: String, name: String) {
        let worker = mem::replace(
            &mut self.grade,
            Some(Box::new(Worker{worker_type: t, worker_name: name, next_worker: None}))
        );
        self.grade.as_mut().unwrap().next_worker = worker
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        let new_head = mem::replace(&mut self.grade.as_mut()?.next_worker, None);
        let old_worker = mem::replace(&mut self.grade, new_head);
        return Some(old_worker.unwrap().worker_name);
    }

    pub fn search_worker(&self) -> Option<(String, String)> {
        Some((self.grade.clone().unwrap().worker_name, self.grade.clone().unwrap().worker_type))
    }
}
