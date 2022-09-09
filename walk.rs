use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    kids: Vec<Node>,
}

struct Node2 {
    name: String,
    kids: Vec<Rc<Node2>>,
    parent: Option<Weak<RefCell<Node2>>>,
}

fn main() {
    let tree = Rc::new(RefCell::new(Node2 {
        name: "root".to_string(),
        kids: vec![],
        parent: None,
    }));
    tree.borrow_mut().parent = Some(Rc::downgrade(&tree));
}
