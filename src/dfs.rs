use crate::graph::{Graph, NodeIndex};

impl<T> Graph<T> {
    /// Creates a depth-first iterator over the graph, starting from `source`.
    pub fn dfs(&self, source: NodeIndex) -> Dfs<T> {
        Dfs {
            graph: self,
            visited: self.nodes.iter().map(|_| false).collect(),
            stack: vec![source],
        }
    }
}

/// A depth-first iterator over a graph.
pub struct Dfs<'g, T> {
    graph: &'g Graph<T>,
    visited: Vec<bool>,
    stack: Vec<NodeIndex>,
}

impl<'g, T> Dfs<'g, T> {
    fn is_visited(&self, node: NodeIndex) -> bool {
        self.visited[node.0]
    }

    fn visit(&mut self, node: NodeIndex) {
        self.visited[node.0] = true;
        for next in self.graph.successors(node) {
            self.stack.push(next);
        }
    }
}

impl<'g, T> Iterator for Dfs<'g, T> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.stack.pop() {
                None => return None,
                Some(node) if self.is_visited(node) => continue,
                Some(node) => {
                    self.visit(node);
                    return Some(node);
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.graph.nodes().len()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn dfs_values<T>(graph: &Graph<T>, start: T) -> Vec<T>
    where
        T: Copy + PartialEq,
    {
        graph
            .dfs(graph.find_node(&start).unwrap())
            .map(|node| graph.node_value(node))
            .copied()
            .collect()
    }

    #[test]
    fn dfs_one_node() {
        let mut graph = Graph::new();
        let node = graph.add_node(1);
        assert_eq!(graph.dfs(node).collect::<Vec<_>>(), [node]);
    }

    #[test]
    fn dfs_two_nodes() {
        let graph = graph![1 -> 2, 2 -> 1];
        assert_eq!(dfs_values(&graph, 1), [1, 2]);
    }

    #[test]
    fn dfs_shallow_graph() {
        let graph = graph![1 -> 2, 1 -> 3, 1 -> 4];
        assert_eq!(dfs_values(&graph, 1), [1, 2, 3, 4]);
    }

    #[test]
    fn dfs_deep_graph() {
        let graph = graph![1 -> 2, 2 -> 3, 3 -> 4];
        assert_eq!(dfs_values(&graph, 1), [1, 2, 3, 4]);
    }

    #[test]
    fn dfs_is_depth_first() {
        let graph = graph! {
            'A' -> 'B', 'A' -> 'C', 'A' -> 'E',
            'B' -> 'A', 'B' -> 'D', 'B' -> 'F',
            'C' -> 'A', 'C' -> 'G',
            'D' -> 'B',
            'E' -> 'A', 'E' -> 'F',
            'F' -> 'B', 'F' -> 'E',
            'G' -> 'C'
        };
        assert_eq!(dfs_values(&graph, 'A'), ['A', 'B', 'D', 'F', 'E', 'C', 'G']);
    }
}
