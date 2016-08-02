pub struct BST {
    root: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    val: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T> BST<T> {
    pub fn new() -> Self {
        BST { root: None }
    }
}

impl<T> BST<T> {
    pub fn insert(&mut self, val: T) -> bool {
        let new_node = Box::new(Node {
            val: val,
            left: None,
            right: None,
        });
    }
}

impl<T> BST<T> {
    pub fn search(&self, item: T) -> bool {
        notimplemented!();
    }
}
