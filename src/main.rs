use core::borrow::BorrowMut;
use std::rc::Rc;

mod binary_search_tree;
mod parental_binary_search_tree;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct TestContainer {
    boxer: Box<Test>,
}

#[derive(Debug)]
struct TestRc {
    rcer: Rc<Test>,
}

#[derive(Debug)]
struct Test {
    value: Option<usize>,
}

#[test]
fn test_box() {
    let mut raw_box = Box::new(Test { value: None });
    raw_box.value.replace(1);

    let mut box_container = TestContainer { boxer: raw_box };
    box_container.boxer.value.replace(3);

    println!("{:?}", box_container);
}


#[test]
fn test_rc() {
    let mut raw_box = Rc::new(Test { value: None });
    //raw_box.as_mut().value.replace(1);

   // let mut b = TestRc { a: b };
   // b.a.as_mut().a.replace(2);

    println!("{:?}", raw_box);
}