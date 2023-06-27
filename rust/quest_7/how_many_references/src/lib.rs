#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub use std::rc::Rc;

pub struct Node {
    pub value: Vec<Rc<String>>,
}

impl Node {
    pub fn new(value: Vec<Rc<String>>) -> Node {
        Node { value: value }
    }

    pub fn add_ele(&mut self, v: Rc<String>) {
        self.value.push(v)
    }

    pub fn rm_all_ref(&mut self, v: Rc<String>) {
        self.value.retain(|k| !Rc::ptr_eq(k, &v))
    }
}

pub fn how_many_references(value: &Rc<String>) -> usize {
    Rc::strong_count(value)
}