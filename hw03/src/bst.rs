type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T: Ord> {
    val: T,
    left: Link<T>,
    right: Link<T>,
}

#[derive(Debug)]
pub struct BST<T: Ord> {
    root: Link<T>,
}

impl<T: Ord> BST<T> {
    pub fn new() -> Self {
        BST { root: None }
    }
}

impl<T: Ord> BST<T> {
    pub fn insert(&mut self, val: T) -> bool {
        let new_node = Box::new(Node {
            val: val,
            left: None,
            right: None,
        });
        if let None = self.root.as_mut() {
                self.root = Some(new_node);
                return true;
        }
        let mut cur_link = self.root.as_mut();
        while let Some(mut box_node) = cur_link {
            if new_node.val == box_node.val {
                return false;
            }
            if new_node.val < box_node.val {
                match box_node.left.as_mut() {
                    None => {
                        box_node.left = Some(new_node);
                        return true;
                    },
                    Some(_) => {
                        cur_link = box_node.left.as_mut();
                    }
                }
            } else {
                match box_node.right.as_mut() {
                    None => {
                        box_node.right = Some(new_node);
                        return true;
                    },
                    Some(_) => {
                        cur_link = box_node.right.as_mut();
                    }
                }
            }
        }
        false
    }
}

impl<T: Ord> BST<T> {
    pub fn search(&self, val: T) -> bool {
	let mut cur_link = self.root.as_ref();
	while let Some(box_node) = cur_link {
            if val == box_node.val  {
                return true;
            }
            if val < box_node.val {
                cur_link = box_node.left.as_ref();
            } else {
                cur_link = box_node.right.as_ref();
            }
	}
        false
    }
}

// Iterators

// IntoIter
pub struct IntoIter<T: Ord>(BST<T>);

impl<T: Ord> BST<T> {
    pub fn into_iter(self) -> IntoIter<T> {
	IntoIter(self)
    }
}

impl<T: Ord> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
	self.0.root.take().map(|box_node| {
	    let node = *box_node;
	    self.0.root = node.right;
	    node.val
	})
    }
}

//Iter
pub struct Iter<'a, T: 'a + Ord> {
    next: Option<&'a Node<T>>,
}

impl<T: Ord> BST<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
	Iter { next: self.root.as_ref().map(|box_node| &**box_node) }
    }
}

impl<'a, T: Ord> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
	self.next.map(|node| {
	    self.next = node.right.as_ref().map(|box_node| &**box_node);
	    &node.val
	})
    }
}

//IterMut
pub struct IterMut<'a, T: 'a + Ord> {
    next: Option<&'a mut Node<T>>,
}

impl<T: Ord> BST<T> {
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
	IterMut { next: self.root.as_mut().map(|box_node| &mut **box_node) }
    }
}

impl<'a, T: Ord> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
	self.next.take().map(|node| {
	    self.next = node.right.as_mut().map(|box_node| &mut **box_node);
	    &mut node.val
	})
    }
}

#[cfg(test)]
mod test {
    use super::BST;

    #[test]
    fn basics() {
        let mut bst = BST::new();

        assert_eq!(bst.search(2), false);

        assert_eq!(bst.insert(2), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.insert(2), false);
        println!("{:#?}", bst);

        assert_eq!(bst.insert(3), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(3), true);
        println!("{:#?}", bst);

        assert_eq!(bst.insert(1), true);
        assert_eq!(bst.insert(8), true);
        assert_eq!(bst.insert(7), true);
        assert_eq!(bst.search(1), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        println!("{:#?}", bst);
    }

    #[test]
    fn into_iter() {
	let mut bst = BST::new();
	bst.insert(1); bst.insert(2); bst.insert(3);

	let mut iter = bst.into_iter();
	assert_eq!(iter.next(), Some(1));
	assert_eq!(iter.next(), Some(2));
	assert_eq!(iter.next(), Some(3));
    }

    #[test]
    fn iter() {
	let mut bst = BST::new();
	bst.insert(1); bst.insert(2); bst.insert(3);

	let mut iter = bst.iter();
	assert_eq!(iter.next(), Some(&1));
	assert_eq!(iter.next(), Some(&2));
	assert_eq!(iter.next(), Some(&3));
    }

    #[test]
    fn iter_mut() {
	let mut bst = BST::new();
	bst.insert(1); bst.insert(2); bst.insert(3);

	let mut iter = bst.iter_mut();
	assert_eq!(iter.next(), Some(&mut 1));
	assert_eq!(iter.next(), Some(&mut 2));
	assert_eq!(iter.next(), Some(&mut 3));
    }
}
