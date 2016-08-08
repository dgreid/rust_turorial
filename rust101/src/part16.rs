use std::ptr;
use std::mem;
use std::marker::PhantomData;

type NodeLink<T> = *mut Node<T>;

struct Node<T> {
    data: T,
    next: NodeLink<T>,
    prev: NodeLink<T>,
}

pub struct List<T> {
    head: NodeLink<T>,
    tail: NodeLink<T>,
    marker: PhantomData<T>,
}

unsafe fn raw_into_box<T>(r: *mut T) -> Box<T> {
    mem::transmute(r)
}

fn box_into_raw<T>(b: Box<T>) -> *mut T {
    unsafe {mem::transmute(b)}
}

impl<T> List<T> {
    fn new() -> Self {
	List { head: ptr::null_mut(), tail: ptr::null_mut(), marker: PhantomData }
    }

    pub fn push_front(&mut self, t: T) {
	let new_box = Box::new(Node { data: t, next: self.head, prev: ptr::null_mut() });
	let new = box_into_raw(new_box);

	if self.head.is_null() {
	    self.tail = new;
	} else {
	    unsafe { (*self.head).prev = new; }
	}
	self.head = new;
    }

    pub fn pop_front(&mut self) -> Option<T> {
	if self.head.is_null() {
	    None
	} else {
	    let b = unsafe { raw_into_box(self.head) };
	    if self.head == self.tail {
		self.tail = ptr::null_mut();
	    }
	    unsafe { self.head = (*self.head).next; }
	    if !self.head.is_null() {
		unsafe { (*self.head).prev = ptr::null_mut(); }
	    }
	    Some(b.data)
	}
    }

    pub fn push_back(&mut self, t: T) {
	let new_box = Box::new(Node { data: t, next: ptr::null_mut(), prev: self.tail });
	let new = box_into_raw(new_box);

	if self.tail.is_null() {
	    self.head = new;
	} else {
	    unsafe { (*self.tail).next = new; }
	}
	self.tail = new;
    }

    pub fn pop_back(&mut self) -> Option<T> {
	if self.tail.is_null() {
	    None
	} else {
	    let b = unsafe { raw_into_box(self.tail) };
	    if self.head == self.tail {
		self.head = ptr::null_mut();
	    }
	    unsafe { self.tail = (*self.tail).prev; }
	    if !self.tail.is_null() {
		unsafe { (*self.tail).next = ptr::null_mut(); }
	    }
	    Some(b.data)
	}
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_push_pop() {
	let mut list = List::new();

	list.push_back(4);
	assert_eq!(list.pop_back(), Some(4));
	assert_eq!(list.pop_back(), None);
	assert_eq!(list.pop_back(), None);

	list.push_front(3);
	assert_eq!(list.pop_front(), Some(3));
	assert_eq!(list.pop_front(), None);
	assert_eq!(list.pop_front(), None);

	list.push_back(4);
	list.push_back(5);
	list.push_front(2);
	list.push_front(1);
	assert_eq!(list.pop_back(), Some(5));
	assert_eq!(list.pop_back(), Some(4));
	assert_eq!(list.pop_front(), Some(1));
	assert_eq!(list.pop_front(), Some(2));
	assert_eq!(list.pop_front(), None);
	assert_eq!(list.pop_back(), None);
    }
}
