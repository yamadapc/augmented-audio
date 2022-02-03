use daggy::NodeIndex;
use std::collections::HashMap;

pub struct DependencyGraph {
    graph: daggy::Dag<String, ()>,
    indexes: HashMap<String, NodeIndex>,
}

impl Default for DependencyGraph {
    fn default() -> Self {
        DependencyGraph {
            graph: Default::default(),
            indexes: Default::default(),
        }
    }
}

impl DependencyGraph {
    pub fn add_crate(&mut self, id: &str) {
        let idx = self.graph.add_node(id.into());
        self.indexes.insert(id.into(), idx);
    }

    pub fn has_crate(&self, id: &str) -> bool {
        self.indexes.contains_key(id)
    }

    pub fn add_dependency(&mut self, pkg: &str, dep: &str) {
        let idx1 = self.indexes[pkg];
        let idx2 = self.indexes[dep];
        self.graph.add_edge(idx1, idx2, ()).unwrap();
    }

    /// Sort dependencies for processing. Dependencies are ordered such that crates with no
    /// dependencies are listed first
    pub fn order_crates(&self) -> Vec<String> {
        let mut sorted_graph = daggy::petgraph::algo::toposort(&self.graph, None).unwrap();

        sorted_graph.reverse();
        sorted_graph
            .iter()
            .map(|idx| self.graph.node_weight(*idx).unwrap().to_string())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_order_crates() {
        let mut graph = DependencyGraph::default();
        graph.add_crate("crate-a");
        graph.add_crate("crate-b");
        graph.add_crate("crate-c");
        graph.add_crate("crate-d");

        graph.add_dependency("crate-a", "crate-b");
        graph.add_dependency("crate-a", "crate-c");
        graph.add_dependency("crate-b", "crate-c");
        graph.add_dependency("crate-c", "crate-d");

        let result = graph.order_crates();
        assert_eq!(result, vec!["crate-d", "crate-c", "crate-b", "crate-a"])
    }
}
