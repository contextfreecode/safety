import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

public class Walk {
    class Node {
        String name = "";
        List<Node> kids = new ArrayList<>();
        // Node parent;
        Optional<Node> parent = Optional.empty();

        Node(String name) {
            this.name = name;
        }

        Node(String name, List<Node> kids) {
            this(name);
            this.kids = new ArrayList<>(kids);
        }
    }

    void initParents(Node tree) {
        for (var kid : tree.kids) {
            kid.parent = Optional.of(tree);
            initParents(kid);
        }
    }

    interface WalkAction {
        void perform(Node node, int depth);
    }

    void walk(Node tree, WalkAction action, int depth) {
        action.perform(tree, depth);
        for (int i = 0; i < tree.kids.size(); i += 1) {
            var kid = tree.kids.get(i);
            walk(kid, action, depth + 1);
        }
    }

    void walk(Node tree, WalkAction action) {
        walk(tree, action, 0);
    }

    void print(Node tree) {
        walk(tree, (node, depth) -> {
            var indentSpec = depth == 0 ? "" : "" + depth * 2;
            System.out.printf("%" + indentSpec + "s%s\n", "", node.name);
        });
    }

    int calcTotalDepth(Node tree) {
        // Or make some fold/reduce walk.
        var total = new int[] { 0 };
        walk(tree, (node, depth) -> {
            total[0] += depth;
        });
        return total[0];
    }

    Node process(Node intro) {
        var tree = new Node("root", List.of(
                intro,
                new Node("one", List.of(
                        new Node("two"),
                        new Node("three"))),
                new Node("four")));
        initParents(tree);
        // Test pointer stability.
        var internalIntro = tree.kids.get(0);
        tree.kids.add(new Node("outro"));
        System.out.println(internalIntro.name);
        // Print and calculate.
        print(tree);
        var totalDepth = 0;
        for (var i = 0; i < 200_000; i += 1) {
            totalDepth += calcTotalDepth(tree);
        }
        System.out.printf("Total depth: %s\n", totalDepth);
        return tree;
    }

    void run() {
        var intro = new Node("intro");
        process(intro);
        System.out.println(intro.parent.get().name);
    }

    public static void main(String[] args) {
        new Walk().run();
    }
}
