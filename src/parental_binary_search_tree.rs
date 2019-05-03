use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use std::cell::RefCell;

type Node<K, V> = Option<Rc<RefCell<NodeBody<K, V>>>>;
type Traced<K, V> = Rc<RefCell<NodeBody<K, V>>>;

#[derive(Debug)]
pub struct ParentalBinarySearchTree<K: Ord + Debug, V: Debug> {
    root: Node<K, V>,
    length: u32,
}

struct NodeBody<K: Ord + Debug, V: Debug> {
    k: K,
    v: V,
    p: Node<K, V>,
    left: Node<K, V>,
    right: Node<K, V>,
}

impl<K: Ord + Debug, V: Debug> NodeBody<K, V> {
    fn new(p: Node<K, V>, k: K, v: V) -> Node<K, V> {
        Some(Rc::new(RefCell::new(Self { k, v, p, left: None, right: None })))
    }
}

impl<K: Ord + Debug, V: Debug> Debug for NodeBody<K, V> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut s = format!("Tree {{ k: {:?}, v: {:?}", self.k, self.v);

        if let Some(p) = &self.p {
            s.push_str(&format!(", p: {:?}", p.as_ref().borrow().k));
        }
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


impl<K: Ord + Debug, V: Debug> ParentalBinarySearchTree<K, V> {
    pub fn new() -> Self {
        Self {
            root: None,
            length: 0,
        }
    }

    pub fn insert(&mut self, k: K, v: V) {
        let root = self.root.take();
        self.length += 1;
        self.root = self.insert_support(root, None, k, v);
    }

    fn insert_support(&mut self, root: Node<K, V>, p: Node<K, V>, k: K, v: V) -> Node<K, V> {
        match root {
            None => NodeBody::new(p, k, v),
            Some(node) => {
                {
                    let p = Some(node.clone());
                    let mut inner = node.as_ref().borrow_mut();
                    if k < inner.k {
                        inner.left = self.insert_support(inner.left.take(), p, k, v);
                    } else {
                        inner.right = self.insert_support(inner.right.take(), p, k, v);
                    }
                }
                Some(node)
            }
        }
    }

    fn trace(&self) -> Vec<Traced<K, V>> {
        let mut v = vec![];
        self.trace_support(&self.root, &mut v);
        v
    }

    fn trace_support(&self, root: &Node<K, V>, result: &mut Vec<Traced<K, V>>) {
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
    let mut t = ParentalBinarySearchTree::new();

    t.insert(11, 11);
    t.insert(21, 21);
    t.insert(1, 1);
    t.insert(22, 22);
    t.insert(23, 23);

    println!("{:?}", t.trace().iter().map(|n| n.as_ref().borrow().k).collect::<Vec<u32>>());
}