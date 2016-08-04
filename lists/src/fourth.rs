use std::rc::Rc;
use std::cell::{Ref, RefMut, RefCell};

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
    pub fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T> List<T> {
    pub fn push_front(&mut self, val: T) {
        let new_head = Node::new(val);
	match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head.clone());
            },
            None => {
                self.tail = Some(new_head.clone());
            }
        }
        self.head = Some(new_head.clone());
    }
}

impl<T> List<T> {
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(head_next) => {
                    head_next.borrow_mut().prev.take();
                    self.head = Some(head_next.clone());
                },
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().val
        })
    }
}

impl<T> List<T> {
    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|head| {
            Ref::map(head.borrow(), |node| &node.val)
        })
    }
}

impl<T> List<T> {
    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head.as_ref().map(|head| {
            RefMut::map(head.borrow_mut(), |node| &mut node.val)
        })
    }
}

impl<T> List<T> {
    pub fn push_back(&mut self, val: T) {
        let new_tail = Node::new(val);
	match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail.clone());
            },
            None => {
                self.head = Some(new_tail.clone());
            }
        }
        self.tail = Some(new_tail.clone());
    }
}

impl<T> List<T> {
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(tail_prev) => {
                    tail_prev.borrow_mut().next.take();
                    self.tail = Some(tail_prev.clone());
                },
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().val
        })
    }
}

impl<T> List<T> {
    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|tail| {
            Ref::map(tail.borrow(), |node| &node.val)
        })
    }
}

impl<T> List<T> {
    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail.as_ref().map(|tail| {
            RefMut::map(tail.borrow_mut(), |node| &mut node.val)
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        println!("dg-- 0");
        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        println!("dg-- 1");
        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        println!("dg-- 2");
        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        println!("dg-- 3");
        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        println!("dg-- 4");
        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        println!("dg-- 5");
        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        println!("dg-- 6");
        // ---- back -----

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        println!("dg-- 7");
        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        println!("dg-- 8");
        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        println!("dg-- 8a");
        assert_eq!(list.pop_back(), Some(2));

        println!("dg-- 9");
        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        println!("dg-- 10");
        // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        println!("dg-- 11");
        // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());

        list.push_front(1); list.push_front(2); list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
    }
}