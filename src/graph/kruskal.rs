use std::collections::HashMap;

use crate::data_structure::DisjointSet;

pub trait NodeVal: Ord + Copy + Default {}
impl<T: Ord + Copy + Default> NodeVal for T {}

pub type NodeId = usize;

#[derive(Default)]
pub struct Graph<V: NodeVal> {
    pub nodes: Vec<V>,
    pub edges: HashMap<NodeId, Vec<Edge>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Edge {
    #[allow(dead_code)]
    src: NodeId,
    dst: NodeId,
    cost: usize,
}

impl<N: NodeVal> Graph<N> {
    pub fn edges(&self, node_id: NodeId) -> &[Edge] {
        if let Some(e) = self.edges.get(&node_id) {
            e
        } else {
            &[]
        }
    }

    #[allow(dead_code)]
    fn add_edge(&mut self, src: NodeId, dst: NodeId, cost: usize) {
        self.edges
            .entry(src)
            .or_default()
            .push(Edge { src, dst, cost });
    }
}

pub fn kruskal<V: NodeVal>(graph: &Graph<V>) -> Vec<Edge> {
    let mut disjoint_set = DisjointSet::new(graph.nodes.len());

    // sort edges
    let mut all_edges: Vec<Edge> = graph.edges.values().flatten().cloned().collect::<Vec<_>>();
    all_edges.sort_by_key(|e| e.cost);

    let mut mst = Vec::new();
    for edge in all_edges {
        // if the edge does not form a cycle
        if disjoint_set.find(edge.src) != disjoint_set.find(edge.dst) {
            // add it to the forest
            disjoint_set.union(edge.src, edge.dst);
            mst.push(edge);
        }
    }
    mst
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::graph::kruskal::{kruskal, Edge, Graph};

    #[test]
    fn test_empty_graph() {
        let graph: Graph<i32> = Graph::default();
        let mst = kruskal(&graph);
        assert!(mst.is_empty());
    }

    #[test]
    fn graph1() {
        let mut graph = Graph {
            nodes: vec![1, 2, 3, 4, 5],
            edges: Default::default(),
        };
        graph.add_edge(0, 1, 1);
        graph.add_edge(1, 2, 2);
        graph.add_edge(2, 3, 3);
        graph.add_edge(3, 4, 4);
        graph.add_edge(0, 4, 10);
        graph.add_edge(1, 3, 5);
        let mst = kruskal(&graph);

        assert_eq!(mst.len(), 4);
        assert_eq!(mst.iter().map(|e| e.cost).sum::<usize>(), 10);
    }

    #[test]
    fn graph2() {
        let mut graph = Graph {
            nodes: vec![1, 2, 3, 4],
            edges: Default::default(),
        };
        graph.add_edge(0, 1, 1);
        graph.add_edge(1, 2, 2);
        graph.add_edge(2, 3, 3);
        graph.add_edge(3, 0, 4);
        graph.add_edge(0, 2, 10);

        let mst = kruskal(&graph);
        assert_eq!(mst.len(), 3);
        assert_eq!(mst.iter().map(|e| e.cost).sum::<usize>(), 6);
    }

    #[test]
    fn graph3() {
        let mut graph = Graph {
            nodes: vec![0, 1, 2, 3, 4, 5, 6],
            edges: Default::default(),
        };
        graph.add_edge(0, 1, 2);
        graph.add_edge(0, 3, 4);
        graph.add_edge(0, 5, 5);
        graph.add_edge(1, 3, 1);
        graph.add_edge(1, 5, 8);
        graph.add_edge(1, 4, 3);
        graph.add_edge(1, 2, 7);
        graph.add_edge(1, 6, 4);
        graph.add_edge(2, 6, 6);
        graph.add_edge(2, 4, 10);
        graph.add_edge(5, 6, 1);
        graph.add_edge(3, 4, 2);

        let mst = kruskal(&graph);
        assert_eq!(mst.len(), 6);
        assert_eq!(mst.iter().map(|e| e.cost).sum::<usize>(), 16);

        let res = [
            (1, 3, 1),
            (5, 6, 1),
            (0, 1, 2),
            (3, 4, 2),
            (1, 6, 4),
            (2, 6, 6),
        ]
        .iter()
        .map(|(src, dst, cost)| Edge {
            src: *src,
            dst: *dst,
            cost: *cost,
        })
        .collect::<BTreeSet<_>>();
        assert_eq!(BTreeSet::from_iter(mst.into_iter()), res);
    }
}
