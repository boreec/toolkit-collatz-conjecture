use petgraph::graph::DiGraph;

pub struct CollatzTree {
    tree: DiGraph<u128, ()>,
    target: u128,
}

impl CollatzTree {
    pub fn new(n: u128) -> Self {
        return CollatzTree {
            tree: CollatzTree::build_tree(n),
            target: n,
        };
    }

    fn build_tree(n: u128) -> DiGraph<u128, ()> {
        let mut root = DiGraph::<u128, ()>::new();

        root.add_node(1);

        // to do
        root
    }
}
