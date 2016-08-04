use std::rc::Rc;
use std::cell::RefCell;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    pub fn new(val: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            val: val,
            next: None,
            prev: None,
        }))
    }
}

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List{ head: None, tail: None, }
    }
}

impl<T> List<T> {
    pub fn push_front(&mut self, val: T) {
        let mut new_node = Node::new(val);
        if let None = self.head.as_mut() {
            self.head = Some(new_node);
            self.tail = self.head;
            return;
        }
        if let Some(mut old_head) = self.head.take() {
            (**new_node).next = Some(old_head);
            old_head.prev = Some(new_node);
            self.head = Some(new_node);
        }
    }
}
