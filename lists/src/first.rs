use std::mem;

pub struct List {
   head: Link,
}

impl List {
    pub fn new() -> List {
	List { head: Link::Empty }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::Valid(mut box_node) = cur_link {
            cur_link = mem::replace(&mut box_node.next, Link::Empty);
        }
    }
}

impl List {
    pub fn push(&mut self, val: i32) {
        let new_node = Box::new(Node {
            val: val,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::Valid(new_node);
    }
}

impl List {
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::Valid(box_node) => {
                let node = *box_node;
                self.head = node.next;
                Some(node.val)
            }
        }
    }
}

enum Link {
    Empty,
    Valid(Box<Node>),
}

struct Node {
    val: i32,
    next: Link,
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check
        // normal
        // removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push
        // some
        // more
        // just
        // to
        // make
        // sure
        // nothing's
        // corrupted
        list.push(4);
        list.push(5);

        // Check
        // normal
        // removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check
        // exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
