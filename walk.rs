struct Node {
    name: String,
    kids: Vec<Node>,
    // parent: Option<&'a Node<'a>>,
}

impl Node {
    fn leaf(name: &str) -> Node {
        Node::parent(name, vec![])
    }

    fn parent(name: &str, kids: Vec<Node>) -> Node {
        Node {
            name: name.into(),
            kids,
            // parent: None,
        }
    }
}

fn walk_depth<Action>(tree: &Node, action: &Action, depth: usize)
where
    Action: Fn(&Node, usize),
{
    action(tree, depth);
    // for kid in &tree.kids {
    for i in 0..tree.kids.len() {
        let kid = &tree.kids[i];
        walk_depth(kid, action, depth + 1);
    }
}

fn walk<Action: Fn(&Node, usize)>(tree: &Node, action: &Action) {
    walk_depth(tree, action, 0);
}

fn process() {
    // -> &'a Node {
    let tree = Node::parent(
        "root",
        vec![
            Node::parent("one", vec![Node::leaf("two"), Node::leaf("three")]),
            Node::leaf("four"),
        ],
    );
    // let node = &tree.kids[0];
    // tree.kids.push(Node::leaf("five"));
    // println!("{}", node.name);
    // tree.parent = Some(&tree);
    walk(&tree, &|node, depth| {
        println!("{:depth$}{name}", "", depth = 2 * depth, name = node.name);
    });
    // &tree
    // &tree.kids[0]
}

fn main() {
    process();
}
