use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

pub trait NodeVal: Ord + Copy + Default {}
impl<T: Ord + Copy + Default> NodeVal for T {}

pub type NodeId = usize;

#[derive(Default)]
pub struct Graph<V: NodeVal> {
    pub nodes: Vec<V>,
    pub edges: HashMap<NodeId, Vec<Edge>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
        self.edges.entry(dst).or_default().push(Edge {
            src: dst,
            dst: src,
            cost,
        });
    }
}

pub fn prim<V: NodeVal>(graph: &Graph<V>) -> Vec<Edge> {
    let mut mst = vec![];
    let mut q = BinaryHeap::new();
    let mut seen = HashSet::new();
    // start with an arbitrary node, push all its edges
    for &e in graph.edges(0) {
        q.push(Reverse((e.cost, e)));
    }
    seen.insert(0);

    while let Some(e) = q.pop() {
        let (_, edge) = e.0;
        if !seen.insert(edge.dst) {
            continue;
        }
        mst.push(edge);
        if mst.len() == graph.nodes.len() {
            break;
        }

        for &edge in graph.edges(edge.dst) {
            if !seen.contains(&edge.dst) {
                q.push(Reverse((edge.cost, edge)));
            }
        }
    }
    mst
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::graph::prim::{prim, Edge, Graph};

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
        let mst = prim(&graph);

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

        let mst = prim(&graph);
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

        let mst = prim(&graph);
        assert_eq!(mst.len(), 6);
        assert_eq!(mst.iter().map(|e| e.cost).sum::<usize>(), 16);

        let res = [
            (0, 1, 2),
            (1, 3, 1),
            (3, 4, 2),
            (1, 6, 4),
            (6, 5, 1),
            (6, 2, 6),
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
