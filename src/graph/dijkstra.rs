use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap, HashMap},
};

pub trait NodeVal: Ord + Copy + Default {}
impl<T: Ord + Copy + Default> NodeVal for T {}

pub type NodeId = usize;

#[derive(Default)]
pub struct Graph<V: NodeVal> {
    pub nodes: Vec<V>,
    pub edges: HashMap<NodeId, Vec<Edge>>,
}

pub struct Edge {
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

    fn add_edge(&mut self, src: NodeId, dst: NodeId, cost: usize) {
        self.edges
            .entry(src)
            .or_default()
            .push(Edge { src, dst, cost });
    }
}

pub fn dijkstra<V: NodeVal>(
    graph: &Graph<V>,
    src: NodeId,
) -> BTreeMap<NodeId, (Option<NodeId>, usize)> {
    let mut prevs = BTreeMap::new();
    let mut queue = BinaryHeap::new();

    // NOTE: the order of the tuple matters
    queue.push(Reverse((0, src)));
    prevs.insert(src, (None, 0));

    for i in 0..graph.nodes.len() {
        if i != src {
            prevs.insert(i, (None, usize::MAX));
        }
    }

    while let Some(entry) = queue.pop() {
        let (node_cost, node_id) = entry.0;
        for edge in graph.edges(node_id) {
            if edge.dst == src {
                continue;
            }
            let new_cost = node_cost + edge.cost;
            let (prev_node, prev_cost) = prevs[&edge.dst];

            // NOTE: the ideal solution would be to populate the heap with usize::MAX priority
            // during initialization, then update the cost of the nodes in the priority queue
            // using decrease_key. However, Rust's BinaryHeap does not support this operation.
            // We accept the duplication of nodes in the priority queue as a tradeoff
            if new_cost < prev_cost || prev_node.is_none() {
                prevs.insert(edge.dst, (Some(node_id), new_cost));
                queue.push(Reverse((new_cost, edge.dst)));
            }
        }
    }
    prevs
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{dijkstra, Graph};

    #[test]
    fn graph1() {
        let mut graph = Graph::default();
        for i in 0..5 {
            graph.nodes.push(i);
        }

        graph.add_edge(0, 2, 12);
        graph.add_edge(0, 3, 60);
        graph.add_edge(1, 0, 10);
        graph.add_edge(2, 1, 20);
        graph.add_edge(2, 3, 32);
        graph.add_edge(4, 0, 7);

        let mut dists_a = BTreeMap::new();
        dists_a.insert(0, (None, 0));
        dists_a.insert(2, (Some(0), 12));
        dists_a.insert(3, (Some(2), 44));
        dists_a.insert(1, (Some(2), 32));
        dists_a.insert(4, (None, usize::MAX));
        assert_eq!(dijkstra(&graph, 0), dists_a);

        let mut dists_b = BTreeMap::new();
        dists_b.insert(1, (None, 0));
        dists_b.insert(0, (Some(1), 10));
        dists_b.insert(2, (Some(0), 22));
        dists_b.insert(3, (Some(2), 54));
        dists_b.insert(4, (None, usize::MAX));
        assert_eq!(dijkstra(&graph, 1), dists_b);

        let mut dists_c = BTreeMap::new();
        dists_c.insert(2, (None, 0));
        dists_c.insert(1, (Some(2), 20));
        dists_c.insert(3, (Some(2), 32));
        dists_c.insert(0, (Some(1), 30));
        dists_c.insert(4, (None, usize::MAX));
        assert_eq!(dijkstra(&graph, 2), dists_c);

        let mut dists_d = BTreeMap::new();
        dists_d.insert(3, (None, 0));
        dists_d.insert(0, (None, usize::MAX));
        dists_d.insert(1, (None, usize::MAX));
        dists_d.insert(2, (None, usize::MAX));
        dists_d.insert(4, (None, usize::MAX));
        assert_eq!(dijkstra(&graph, 3), dists_d);

        let mut dists_e = BTreeMap::new();
        dists_e.insert(4, (None, 0));
        dists_e.insert(0, (Some(4), 7));
        dists_e.insert(2, (Some(0), 19));
        dists_e.insert(3, (Some(2), 51));
        dists_e.insert(1, (Some(2), 39));
        assert_eq!(dijkstra(&graph, 4), dists_e);
    }

    // https://imagedelivery.net/CLfkmk9Wzy8_9HRyug4EVA/482027d5-fb4e-4a3c-d710-ec60cbead600/sharpen=1
    #[test]
    fn graph2() {
        let mut graph = Graph::default();
        for i in 0..5 {
            graph.nodes.push(i);
        }

        graph.add_edge(0, 1, 10);
        graph.add_edge(0, 2, 3);
        graph.add_edge(1, 3, 2);
        graph.add_edge(2, 1, 4);
        graph.add_edge(2, 3, 8);
        graph.add_edge(2, 4, 2);
        graph.add_edge(3, 4, 5);

        let mut expected = BTreeMap::new();
        expected.insert(0, (None, 0));
        expected.insert(1, (Some(2), 7));
        expected.insert(2, (Some(0), 3));
        expected.insert(3, (Some(1), 9));
        expected.insert(4, (Some(2), 5));
        assert_eq!(dijkstra(&graph, 0), expected);
    }
}
