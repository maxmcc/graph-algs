/// A graph data structure.
///
/// The implementation of this graph is inspired by [Niko Matsakis's blog post][blog].
///
/// [blog]: http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Graph<T> {
    pub(crate) nodes: Vec<Node<T>>,
    pub(crate) edges: Vec<Edge>,
}

impl<T> Graph<T> {
    /// Creates a new empty graph.
    pub fn new() -> Self {
        Graph {
            nodes: vec![],
            edges: vec![],
        }
    }

    /// Adds a new node to the graph, and returns a `NodeIndex` representing it.
    pub fn add_node(&mut self, value: T) -> NodeIndex {
        let index = NodeIndex(self.nodes.len());
        self.nodes.push(Node {
            value: value,
            first_outgoing_edge: None,
        });
        index
    }

    /// Adds a new edge to the graph, and returns an `EdgeIndex` representing it.
    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) -> EdgeIndex {
        let edge_index = EdgeIndex(self.edges.len());
        let node_data = &mut self.nodes[source.0];
        self.edges.push(Edge {
            target: target,
            next_outgoing_edge: node_data.first_outgoing_edge,
        });
        node_data.first_outgoing_edge = Some(edge_index);
        edge_index
    }

    /// Returns an iterator over the node indices and values of the graph.
    pub fn nodes(&self) -> impl ExactSizeIterator<Item = (NodeIndex, &T)> {
        self.nodes
            .iter()
            .enumerate()
            .map(|(index, node)| (NodeIndex(index), &node.value))
    }

    /// Returns an iterator over the edge indices of the graph.
    pub fn edges(&self) -> impl ExactSizeIterator<Item = EdgeIndex> {
        (0..self.edges.len()).map(EdgeIndex)
    }

    /// Returns an iterator over the successors of a given node.
    pub fn successors(&self, source: NodeIndex) -> Successors<T> {
        let first_outgoing_edge = self.nodes[source.0].first_outgoing_edge;
        Successors {
            graph: self,
            current_edge_index: first_outgoing_edge,
        }
    }

    pub fn node_value(&self, index: NodeIndex) -> &T {
        &self.nodes[index.0].value
    }

    pub fn node_value_mut(&mut self, index: NodeIndex) -> &mut T {
        &mut self.nodes[index.0].value
    }
}

impl<T: PartialEq> Graph<T> {
    /// Finds the `NodeIndex` corresponding to the given value in the graph.
    ///
    /// If the graph does not contain `value`, this function returns `None`. If the graph contains
    /// `value` in multiple nodes, the node index returned is arbitrary.
    pub fn find_node(&self, value: &T) -> Option<NodeIndex> {
        self.nodes.iter().enumerate().find_map(|(index, node)| {
            if node.value == *value {
                Some(NodeIndex(index))
            } else {
                None
            }
        })
    }
}

impl<T: PartialEq> std::iter::FromIterator<(T, T)> for Graph<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (T, T)>,
    {
        let mut graph = Graph::new();
        for (source, target) in iter {
            let source = graph
                .find_node(&source)
                .unwrap_or_else(|| graph.add_node(source));
            let target = graph
                .find_node(&target)
                .unwrap_or_else(|| graph.add_node(target));
            graph.add_edge(source, target);
        }
        graph
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct NodeIndex(pub(crate) usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct Node<T> {
    value: T,
    first_outgoing_edge: Option<EdgeIndex>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct EdgeIndex(pub(crate) usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct Edge {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
}

pub struct Successors<'g, T> {
    graph: &'g Graph<T>,
    current_edge_index: Option<EdgeIndex>,
}

impl<'g, T> Iterator for Successors<'g, T> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let edge = &self.graph.edges[edge_index.0];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge.target)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.graph.nodes.len() - 1))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_node_empty() {
        let graph = Graph::new();
        assert_eq!(graph.find_node(&1), None);
    }

    #[test]
    fn find_node_singleton() {
        let mut graph = Graph::new();
        let node = graph.add_node(1);
        assert_eq!(graph.find_node(&1), Some(node));
        assert_eq!(graph.find_node(&2), None);
    }

    #[test]
    fn find_node_many() {
        let mut graph = Graph::new();
        let one = graph.add_node(1);
        let two = graph.add_node(2);
        assert_eq!(graph.find_node(&1), Some(one));
        assert_eq!(graph.find_node(&2), Some(two));
        assert_eq!(graph.find_node(&3), None);
    }

    #[test]
    fn test_add_multiple_nodes() {
        let mut graph = Graph::new();
        for _ in 0..5 {
            graph.find_node(&1).unwrap_or_else(|| graph.add_node(1));
        }
        assert!(graph.find_node(&1).is_some());
        assert_eq!(graph.nodes().len(), 1);
    }
}
