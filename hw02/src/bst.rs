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
}
