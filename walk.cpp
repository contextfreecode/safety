#include <functional>
#include <iostream>
#include <memory>
#include <optional>
#include <string>
#include <vector>

struct Node {
    std::string name;
    std::vector<Node> kids;
    Node* parent; // TODO = nullptr;
    // std::optional<std::reference_wrapper<Node>> parent;
};

auto init_parents(Node& tree) -> void {
    for (auto& kid : tree.kids) {
        kid.parent = &tree;
        // kid.parent = tree;
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

auto print(const Node& tree) -> void {
    walk(tree, [](const auto& node, auto depth) {
        std::cout << std::string(2 * depth, ' ') << node.name << "\n";
    });
}

auto calc_total_depth(const Node& tree) -> int {
    auto total = 0;
    walk(tree, [&total](const auto& _, auto depth) {
        total += depth;
    });
    return total;
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
    // Test pointer stability.
    auto& internal_intro = tree.kids[0];
    // tree.kids.push_back({"outro"});
    print(internal_intro);
    // Print and calculate.
    print(tree);
    auto total_depth = 0;
    for (auto i = 0; i < 200'000; i += 1) {
        total_depth += calc_total_depth(tree);
    }
    std::cout << "Total depth: " << total_depth << "\n";
    return internal_intro;
}

auto main() -> int {
    auto intro = Node {"intro"};
    // auto& internal_intro =
    process(intro);
    // print(intro);
    // std::cout << intro.parent->name << "\n";
    // std::cout << intro.parent.value().get().name << "\n";
}
