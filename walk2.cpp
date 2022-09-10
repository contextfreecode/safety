#include <functional>
#include <iostream>
#include <memory>
#include <optional>
#include <string>
#include <vector>

struct Node {
    std::string name;
    std::vector<std::shared_ptr<Node>> kids;
    std::weak_ptr<Node> parent;
};

auto init_parents(std::shared_ptr<Node> tree) -> void {
    for (auto& kid : tree->kids) {
        kid->parent = tree;
        init_parents(kid);
    }
}

auto walk(
    const Node& tree,
    std::function<void(const Node& node, int depth)> action,
    int depth = 0
) -> void {
    action(tree, depth);
    for (const auto& kid : tree.kids) {
        walk(*kid, action, depth + 1);
    }
}

auto process(std::shared_ptr<Node> intro) -> std::shared_ptr<Node> {
    auto tree = std::make_shared<Node>(Node {"root", {
        intro,
        // {"one", {
        //     {"two"},
        //     {"three"},
        // }},
        // {"four"},
    }});
    init_parents(tree);
    auto internal_intro = tree->kids[0];
    tree->kids.push_back(std::make_shared<Node>(Node {"outro"}));
    std::cout << internal_intro->name << "\n";
    walk(*tree, [](const auto& node, int depth) {
        std::cout << std::string(2 * depth, ' ') << node.name << "\n";
    });
    return tree;
}

auto main() -> int {
    auto intro = std::make_shared<Node>(Node {"intro"});
    auto tree = process(intro);
    std::cout << intro->parent.lock()->name << "\n";
}
