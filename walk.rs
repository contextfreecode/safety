#[derive(Clone)]
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

// fn init_parents(tree: &mut Node) {
//     for kid in &mut tree.kids {
//         kid.parent = Some(&tree);
//         init_parents(kid);
//     }
// }

fn walk_depth<Action>(tree: &Node, action: &mut Action, depth: usize)
where
    Action: FnMut(&Node, usize),
{
    action(tree, depth);
    // for kid in &tree.kids {
    for i in 0..tree.kids.len() {
        let kid = &tree.kids[i];
        walk_depth(kid, action, depth + 1);
    }
}

fn walk<Action: FnMut(&Node, usize)>(tree: &Node, action: &mut Action) {
    walk_depth(tree, action, 0);
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

fn process(intro: &Node) {
    // -> &Node {
    let tree = Node::parent(
        "root",
        vec![
            intro.clone(),
            Node::parent("one", vec![Node::leaf("two"), Node::leaf("three")]),
            Node::leaf("four"),
        ],
    );
    // init_parents(&mut tree);
    // let internal_intro = &tree.kids[0];
    // tree.kids.push(Node::leaf("outro"));
    // println!("{}", internal_intro.name);
    // internal_intro.parent = Some(&tree);
    print_tree(&tree);
    let mut total_depth = 0;
    for _ in 0..200_000 {
        total_depth += calc_total_depth(&tree);
    }
    println!("Total depth: {total_depth}");
    // &tree
    // internal_intro
}

fn main() {
    let intro = Node::leaf("intro");
    process(&intro);
}
