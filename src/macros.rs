#[macro_export]
macro_rules! graph {
    ( $( $x:tt -> $y:tt ),* $(,)? ) => {
        {
            #[allow(unused_mut)]
            let mut g = Graph::new();
            $(
                let x = $x;
                let y = $y;
                let x = g.find_node(&x).unwrap_or_else(|| g.add_node(x));
                let y = g.find_node(&y).unwrap_or_else(|| g.add_node(y));
                g.add_edge(x, y);
            )*
            g
        }
    }
}

#[cfg(test)]
mod test {
    use crate::graph::Graph;
    use maplit::hashset;
    use std::collections::HashSet;

    #[test]
    fn macro_empty() {
        let graph: Graph<i32> = graph![];
        assert_eq!(graph.nodes().len(), 0);
    }

    #[test]
    fn macro_one_edge() {
        let graph = graph![0 -> 1];
        assert_eq!(graph.nodes().len(), 2);
        assert_eq!(graph.edges().len(), 1);
    }

    #[test]
    fn macro_two_edges() {
        let graph = graph![0 -> 1, 1 -> 0];
        assert_eq!(graph.nodes().len(), 2);
        assert_eq!(graph.edges().len(), 2);
    }

    #[test]
    fn macro_many_edges() {
        let graph = graph! {
            0 -> 1, 0 -> 3,
            1 -> 0, 1 -> 2, 1 -> 3,
            2 -> 0,
            3 -> 0, 3 -> 2,
        };
        assert_eq!(graph.nodes().len(), 4);
        assert_eq!(graph.edges().len(), 8);

        let one = graph.find_node(&1).unwrap();
        assert_eq!(
            graph
                .successors(one)
                .map(|node| graph.node_value(node))
                .copied()
                .collect::<HashSet<_>>(),
            hashset![0, 2, 3]
        );
    }
}
