const std = @import("std");
const Allocator = std.mem.Allocator;
const print = @import("std").debug.print;

const Node = struct {
    name: []const u8 = "",
    kids: std.ArrayList(Node),
    parent: ?*Node = null,

    fn leaf(allocator: Allocator, name: []const u8) Node {
        return parent(allocator, name, &.{}) catch unreachable;
    }

    fn parent(allocator: Allocator, name: []const u8, kids: []Node) !Node {
        var kids_list = std.ArrayList(Node).init(allocator);
        try kids_list.ensureTotalCapacityPrecise(kids.len);
        try kids_list.appendSlice(kids);
        return Node{ .name = name, .kids = kids_list };
    }

    fn deinit(self: Node) void {
        // TODO Reverse order to crash!
        for (self.kids.items) |*kid| {
            kid.deinit();
        }
        self.kids.deinit();
    }
};

fn initParents(tree: *Node) void {
    for (tree.kids.items) |*kid| {
        kid.parent = tree;
        initParents(kid);
    }
}

fn walkDepth(tree: Node, comptime action: fn (Node, usize) void, depth: usize) void {
    action(tree, depth);
    for (tree.kids.items) |kid| {
    // var i = @as(usize, 0);
    // while (i < depth) : (i += 1) {
    //     const kid = tree.kids.items[i];
        walkDepth(kid, action, depth + 1);
    }
}

fn walk(tree: Node, comptime action: fn (Node, usize) void) void {
    walkDepth(tree, action, 0);
}

fn printNode(node: Node, depth: usize) void {
    var i = @as(usize, 0);
    while (i < 2 * depth) : (i += 1) {
        print(" ", .{});
    }
    print("{s}\n", .{node.name});
}

fn printTree(tree: Node) void {
    walk(tree, printNode);
}

fn process(intro: *Node) !void {
    const allocator = intro.kids.allocator;
    var tree = try Node.parent(allocator, "root", &.{
        intro.*,
        try Node.parent(allocator, "one", &.{
            Node.leaf(allocator, "two"),
            Node.leaf(allocator, "three"),
        }),
        Node.leaf(allocator, "four"),
    });
    defer tree.deinit();
    initParents(&tree);
    const internal_intro = &tree.kids.items[0];
    // try tree.kids.append(Node.leaf(allocator, "outro"));
    print("{s}\n", .{internal_intro.name});
    printTree(tree);
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    var intro = Node.leaf(allocator, "intro");
    try process(&intro);
}
