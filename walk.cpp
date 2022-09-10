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

auto init_parents(Node& tree) -> void {
    for (auto& kid : tree.kids) {
        kid.parent = &tree;
        init_parents(kid);
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
        const auto& kid = tree.kids[i]; // .at(i)
        walk(kid, action, depth + 1);
    } // TODO Explore indentation warning sans curlies?
}

auto process(Node& intro) -> Node& {
    auto tree = Node {"root", {
        intro,
        {"one", {
            {"two"},
            {"three"},
        }},
        {"four"},
    }};
    init_parents(tree);
    auto& internal_intro = tree.kids[0];
    // tree.kids.push_back({"outro"});
    // std::cout << internal_intro.name << "\n";
    walk(tree, [](const auto& node, int depth) {
        std::cout << std::string(2 * depth, ' ') << node.name << "\n";
    });
    return internal_intro;
}

auto main() -> int {
    auto intro = Node {"intro"};
    process(intro);
    // std::cout << intro.parent->name << "\n";
}
