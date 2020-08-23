use crate::graph::{Graph, NodeIndex, Successors};
use std::collections::{HashSet, VecDeque};

impl<T> Graph<T> {
    /// Creates a breadth-first iterator over the graph, starting from `source`.
    ///
    /// This iterator returns sets of nodes, grouped by their depth from the root node. To iterate
    /// over a sequence of individual nodes, use the [`flatten`] method on the BFS iterator.
    ///
    /// [`flatten`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flatten
    pub fn bfs(&self, source: NodeIndex) -> Bfs<T> {
        use maplit::hashset;
        let mut bfs = Bfs {
            graph: self,
            visited: self.nodes.iter().map(|_| false).collect(),
            queue: VecDeque::new(),
        };
        bfs.queue.push_back(hashset![source]);
        bfs
    }
}

/// A breadth-first iterator over a graph.
pub struct Bfs<'g, T> {
    graph: &'g Graph<T>,
    visited: Vec<bool>,
    queue: VecDeque<HashSet<NodeIndex>>,
}

impl<'g, T> Bfs<'g, T> {
    fn is_visited(&self, node: NodeIndex) -> bool {
        self.visited[node.0]
    }

    fn visit(&mut self, node: NodeIndex) -> Successors<'g, T> {
        self.visited[node.0] = true;
        self.graph.successors(node)
    }
}

impl<'g, T> Iterator for Bfs<'g, T> {
    type Item = HashSet<NodeIndex>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.queue.pop_front() {
            None => None,
            Some(nodes) => {
                let mut next = HashSet::new();
                for &node in nodes.iter() {
                    next.extend(self.visit(node));
                }
                next.retain(|&next| !self.is_visited(next));
                if !next.is_empty() {
                    self.queue.push_back(next);
                }
                Some(nodes)
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
    use maplit::hashset;

    fn bfs_values<T>(graph: &Graph<T>, start: T) -> Vec<HashSet<T>>
    where
        T: Copy + std::hash::Hash + Eq,
    {
        graph
            .bfs(graph.find_node(&start).unwrap())
            .map(|nodes| {
                nodes
                    .iter()
                    .map(|&node| graph.node_value(node))
                    .copied()
                    .collect()
            })
            .collect()
    }

    #[test]
    fn bfs_one_node() {
        let mut graph = Graph::new();
        graph.add_node(1);
        assert_eq!(bfs_values(&graph, 1), [hashset![1]]);
    }

    #[test]
    fn bfs_two_nodes() {
        let graph = graph![1 -> 2, 2 -> 1];
        assert_eq!(bfs_values(&graph, 1), [hashset![1], hashset![2]]);
        assert_eq!(bfs_values(&graph, 2), [hashset![2], hashset![1]]);
    }

    #[test]
    fn bfs_deep_graph() {
        let graph = graph![1 -> 2, 2 -> 3, 3 -> 4];
        assert_eq!(
            bfs_values(&graph, 1),
            [hashset![1], hashset![2], hashset![3], hashset![4]]
        );
    }

    #[test]
    fn bfs_shallow_graph() {
        let graph = graph![1 -> 2, 1 -> 3, 1 -> 4];
        assert_eq!(bfs_values(&graph, 1), [hashset![1], hashset![2, 3, 4]]);
    }

    #[test]
    fn bfs_is_breadth_first() {
        let graph = graph! {
            'A' -> 'B', 'A' -> 'C', 'A' -> 'E',
            'B' -> 'A', 'B' -> 'D', 'B' -> 'F',
            'C' -> 'A', 'C' -> 'G',
            'D' -> 'B',
            'E' -> 'A', 'E' -> 'F',
            'F' -> 'B', 'F' -> 'E',
            'G' -> 'C'
        };
        assert_eq!(
            bfs_values(&graph, 'A'),
            [
                hashset!['A'],
                hashset!['B', 'C', 'E'],
                hashset!['D', 'F', 'G']
            ]
        );
    }
}
