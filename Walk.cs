using System.Linq;
using System.Runtime.InteropServices;

// sealed record class Box<T>(T Value) where T : struct {}

class Walker {
    record struct Node(
        string Name,
        List<Node> Kids
        // Node parent
        // Box<Node>? parent = null
    ) {
        public Node(string name) : this(name, new()) { }
    }

    delegate void WalkAction(Node node, int depth);

    void Walk(Node tree, WalkAction action, int depth = 0) {
        action(tree, depth);
        for (var i = 0; i < tree.Kids.Count; i += 1) {
            var kid = tree.Kids[i];
            Walk(kid, action, depth + 1);
        }
    }

    void Print(Node tree) {
        Walk(tree, (node, depth) => {
            Console.WriteLine($"{"".PadLeft(2 * depth)}{node.Name}");
        });
    }

    int CalcTotalDepth(Node tree) {
        var total = 0;
        Walk(tree, (_, depth) => {
            total += depth;
        });
        return total;
    }

    void Process(Node intro) {
        var tree = new Node("root", new List<Node> {
            intro,
            new("one", new List<Node> {
                new("two"),
                new("three"),
            }),
            new("four"),
        });
        // Test pointer stability.
        // var nodes = CollectionsMarshal.AsSpan(tree.Kids);
        var internalIntro = tree.Kids[0];
        tree.Kids.Add(new("outro"));
        // tree.Kids.Clear();
        // Print(nodes[0]);
        Print(internalIntro);
        // Print tree and calculate.
        Print(tree);
        var totalDepth = 0;
        foreach (var _ in Enumerable.Range(0, 200_000)) {
            totalDepth += CalcTotalDepth(tree);
        }
        Console.WriteLine($"Total depth: {totalDepth}");
    }

    void Run() {
        var intro = new Node("intro");
        Process(intro);
    }

    // static void Main(string[] args) {
    //     new Walker().Run();
    // }
}
