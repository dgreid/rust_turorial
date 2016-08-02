use std::sync::Arc;

type Link<T> = Option<Arc<Node<T>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
}

pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self{
        List { head: None }
    }
}

impl<T> List<T> {
    pub fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(rc_node) = head {
            if let Ok(mut node) = Arc::try_unwrap(rc_node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

impl<T> List<T> {
    pub fn append(&self, val: T) -> List<T> {
        List {
            head: Some(Arc::new(Node { val: val, next: self.head.clone(), })),
        }
    }
}

impl<T> List<T> {
    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }
}

impl<T> List<T> {
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val)
    }
}

// Iter

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|nnode| &**nnode);
            &node.val
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list1 = List::new();
        assert_eq!(list1.head(), None);
        let list2 = list1.append(2).append(3);
        assert_eq!(list2.head(), Some(&3));
        assert_eq!(list2.tail().head(), Some(&2));
        assert_eq!(list2.tail().tail().head(), None);
    }

    #[test]
    fn iters() {
        let list = List::new().append(2).append(3).append(4);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
    }
}
