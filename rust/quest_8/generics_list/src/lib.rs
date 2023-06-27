#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
    pub len: usize,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List{
            head: None,
            len: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        self.head = Some(Node{
            value,
            next: self.head.take().map(Box::new)
        });
        self.len += 1
    }

    pub fn pop(&mut self) {
        if let Some(node) = self.head.take() {
            self.len -= 1;
            self.head = node.next.map(|next| *next);
        };
    }

    pub fn len(&self) -> usize {
        self.len
    }
}