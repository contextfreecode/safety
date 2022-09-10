use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    name: String,
    kids: Vec<Rc<RefCell<Node>>>,
    parent: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn leaf(name: &str) -> Node {
        Node::parent(name, vec![])
    }

    fn parent(name: &str, kids: Vec<Node>) -> Node {
        Node {
            name: name.into(),
            kids: kids
                .into_iter()
                .map(|kid| Rc::new(RefCell::new(kid)))
                .collect(),
            parent: None,
        }
    }
}

fn init_parents(tree: Rc<RefCell<Node>>) {
    for kid in &tree.borrow_mut().kids {
        kid.borrow_mut().parent = Some(Rc::downgrade(&tree));
        init_parents(kid.clone());
    }
}

fn walk_depth<Action>(tree: Rc<RefCell<Node>>, action: &Action, depth: usize)
where
    Action: Fn(&Node, usize),
{
    action(&tree.borrow(), depth);
    let kids = &tree.borrow().kids;
    // for kid in &kids {
    for i in 0..kids.len() {
        let kid = &kids[i];
        walk_depth(kid.clone(), action, depth + 1);
    }
}

fn walk<Action: Fn(&Node, usize)>(tree: Rc<RefCell<Node>>, action: &Action) {
    walk_depth(tree, action, 0);
}

fn process() {
    // -> &'a Node {
    let tree = Rc::new(RefCell::new(Node::parent(
        "root",
        vec![
            Node::parent("one", vec![Node::leaf("two"), Node::leaf("three")]),
            Node::leaf("four"),
        ],
    )));
    init_parents(tree.clone());
    println!(
        "{}",
        tree.borrow().kids[0]
            .borrow()
            .parent
            .as_ref()
            .unwrap()
            .upgrade()
            .unwrap()
            .borrow()
            .name
    );
    // let node = &tree.kids[0];
    // tree.kids.push(Node::leaf("five"));
    // println!("{}", node.name);
    // tree.parent = Some(&tree);
    walk(tree, &|node, depth| {
        println!("{:depth$}{name}", "", depth = 2 * depth, name = node.name);
    });
    // &tree
    // &tree.kids[0]
}

fn main() {
    process();
}
