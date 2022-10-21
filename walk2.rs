use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    name: String,
    kids: Vec<Rc<RefCell<Node>>>,
    parent: Weak<RefCell<Node>>,
}

impl Node {
    fn leaf(name: &str) -> Node {
        Node::parent(name, vec![])
    }

    fn parent(name: &str, kids: Vec<Rc<RefCell<Node>>>) -> Node {
        Node {
            name: name.into(),
            kids: kids,
            parent: Weak::new(),
        }
    }
}

fn rc_ref_cell<Value>(value: Value) -> Rc<RefCell<Value>> {
    Rc::new(RefCell::new(value))
}

fn init_parents(tree: Rc<RefCell<Node>>) {
    for kid in &tree.borrow_mut().kids {
        kid.borrow_mut().parent = Rc::downgrade(&tree);
        init_parents(kid.clone());
    }
}

fn walk_depth<Action>(tree: &Node, action: &mut Action, depth: usize)
where
    Action: FnMut(&Node, usize),
{
    action(tree, depth);
    let kids = &tree.kids;
    // for kid in kids {
    for i in 0..kids.len() {
        let kid = &kids[i];
        walk_depth(&kid.borrow(), action, depth + 1);
    }
}

fn walk<Action: FnMut(&Node, usize)>(tree: &Node, action: &mut Action) {
    walk_depth(&tree, action, 0);
}

fn print_tree(tree: &Node) {
    walk(tree, &mut |node, depth| {
        println!("{:depth$}{name}", "", depth = 2 * depth, name = node.name);
    });
}

fn calc_total_depth(tree: &Node) -> usize {
    let mut total = 0;
    walk(tree, &mut |_node, depth| {
        total += depth;
    });
    total
}

fn process(intro: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    let tree = rc_ref_cell(Node::parent(
        "root",
        vec![
            intro,
            rc_ref_cell(Node::parent(
                "one",
                vec![
                    rc_ref_cell(Node::leaf("two")),
                    rc_ref_cell(Node::leaf("three")),
                ],
            )),
            rc_ref_cell(Node::leaf("four")),
        ],
    ));
    init_parents(tree.clone());
    // Test pointer stability.
    let internal_intro = tree.borrow().kids[0].clone();
    tree.borrow_mut().kids.push(rc_ref_cell(Node::leaf("outro")));
    print_tree(&internal_intro.borrow());
    // Print and calculate.
    print_tree(&tree.borrow());
    let mut total_depth = 0;
    for _ in 0..200_000 {
        total_depth += calc_total_depth(&tree.borrow());
    }
    println!("Total depth: {total_depth}");
    tree
}

fn main() {
    let intro = rc_ref_cell(Node::leaf("intro"));
    let _tree = process(intro.clone());
    println!("{}", intro.borrow().parent.upgrade().unwrap().borrow().name);
}
