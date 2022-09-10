#include <functional>
#include <iostream>
#include <memory>
#include <string>
#include <vector>

struct Node {
    std::string name;
    std::vector<Node> kids;
    // std::weak_ptr<Node> parent; // TODO std::optional?
    Node* parent; // TODO = nullptr; // TODO std::optional?
};

auto init_parents(Node* tree) -> void {
    for (auto& kid : tree->kids) {
        kid.parent = tree;
        init_parents(&kid);
    }
}

auto walk(
    const Node& tree,
    std::function<void(const Node& node, int depth)> action,
    int depth = 0
) -> void {
    action(tree, depth);
    // for (const auto& kid : tree.kids) {
    for (size_t i = 0; i < tree.kids.size(); i += 1) {
        const auto& kid = tree.kids[i];
        walk(kid, action, depth + 1);
    } // TODO Explore indentation warning sans curlies?
}

auto process() -> Node& {
    auto tree = Node {"root", {
        {"one", {
            {"two"},
            {"three"},
        }},
        {"four"},
    }};
    // auto& node = tree.kids[0];
    // tree.kids.push_back({"five"});
    // std::cout << node.name << "\n";
    // std::cout << tree.parent.lock()->name << "\n";
    // std::cout << tree.parent->name << "\n";
    init_parents(&tree);
    walk(tree, [](const auto& node, int depth) {
        std::cout << std::string(2 * depth, ' ') << node.name << "\n";
    });
    return tree.kids[0];
    // return tree;
}

auto main() -> int {
    process();
    // auto& tree = process();
    // Can cause arbitrary output.
    // std::cout << tree.name << "\n";
}
