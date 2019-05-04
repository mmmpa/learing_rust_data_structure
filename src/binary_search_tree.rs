use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use std::cell::RefCell;
use core::borrow::BorrowMut;

type Node<T> = Option<Rc<RefCell<NodeBody<T>>>>;
type Traced<T> = Rc<RefCell<NodeBody<T>>>;

#[derive(Debug)]
struct BinarySearchTree<T: Ord + Debug> {
    root: Node<T>,
    length: u32,
}

struct NodeBody<T: Ord + Debug> {
    value: T,
    left: Node<T>,
    right: Node<T>,
}

impl<T: Ord + Debug> NodeBody<T> {
    fn new(value: T) -> Node<T> {
        Some(Rc::new(RefCell::new(Self { value, left: None, right: None })))
    }
}

impl<T: Ord + Debug> Debug for NodeBody<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut s = format!("Tree {{ v: {:?}", self.value);

        if let Some(_) = self.left {
            s.push_str(&format!(", left: {:?}", self.left));
        }
        if let Some(_) = self.right {
            s.push_str(&format!(", right: {:?}", self.right));
        }

        s.push_str(" }");

        f.write_str(&s)
    }
}


impl<T: Ord + Debug> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self {
            root: None,
            length: 0,
        }
    }

    pub fn insert(&mut self, v: T) {
        let root = self.root.take();
        self.length += 1;
        self.root = self.insert_support(root, v);
    }

    fn insert_support(&mut self, root: Node<T>, v: T) -> Node<T> {
        match root {
            None => NodeBody::new(v),
            Some(mut node) => {
                {
                    let mut inner = node.as_ref().borrow_mut();
                    if v < inner.value {
                        inner.left = self.insert_support(inner.left.take(), v);
                    } else {
                        inner.right = self.insert_support(inner.right.take(), v);
                    }
                }
                Some(node)
            }
        }
    }

    fn trace(&self) -> Vec<Traced<T>> {
        let mut v = vec![];
        self.trace_support(&self.root, &mut v);
        v
    }

    fn trace_support(&self, root: &Node<T>, result: &mut Vec<Traced<T>>) {
        match root {
            None => (),
            Some(node) => {
                let inner = node.as_ref().borrow();

                self.trace_support(&inner.left, result);
                result.push(node.clone());
                self.trace_support(&inner.right, result);
            }
        }
    }
}

#[test]
fn test_insert() {
    let mut t = BinarySearchTree::new();

    t.insert(11);
    t.insert(21);
    t.insert(1);
    t.insert(22);
    t.insert(23);

    println!("{:?}", t.trace().iter().map(|n| n.as_ref().borrow().value).collect::<Vec<u32>>());
}