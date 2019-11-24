use std::rc::Rc;
use std::rc::Weak;

pub struct RingBuffer<T> {
    nodes: Vec<Rc<Node<T>>>,
    head: Option<Rc<Node<T>>>,
    tail: Option<Rc<Node<T>>>,
}

struct Node<T> {
    value: T,
    prev: Weak<Node<T>>,
    next: Weak<Node<T>>,
}

impl<T> RingBuffer<T> {
    fn new() -> Self {
        Self {
            nodes: vec![],
            head: None,
            tail: None,
        }
    }

    fn push_back(&mut self, value: T) {
        let mut n = Node {
            value,
            prev: Weak::new(),
            next: Weak::new(),
        };
        let n_ref = Rc::new(n);

        if self.head.is_none() {
            self.head = Some(Rc::clone(&n_ref));
            self.tail = Some(Rc::clone(&n_ref));
        }

        self.nodes.push(n_ref);


    }
}
