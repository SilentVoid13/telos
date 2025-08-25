use std::collections::{BTreeMap, BinaryHeap, HashMap};

pub trait NodeVal: Ord + Copy + Default {}
impl<T: Ord + Copy + Default> NodeVal for T {}

pub type NodeId = usize;

#[derive(Default)]
pub struct Graph<V: NodeVal> {
    pub nodes: Vec<V>,
    pub edges: HashMap<NodeId, Vec<Edge>>,
}

#[derive(Debug)]
pub struct Edge {
    #[allow(dead_code)]
    src: NodeId,
    dst: NodeId,
    cost: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub struct QueueEntry {
    cost: usize,
    node_id: NodeId,
}

impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // NOTE: we reverse the order to get smaller cost at the top
        other.cost.cmp(&self.cost)
    }
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

pub fn dijkstra<V: NodeVal>(
    graph: &Graph<V>,
    src: NodeId,
) -> BTreeMap<NodeId, (Option<NodeId>, usize)> {
    let mut prevs = BTreeMap::new();
    let mut queue = BinaryHeap::new();

    // NOTE: the order of the tuple matters
    queue.push(QueueEntry {
        cost: 0,
        node_id: src,
    });
    prevs.insert(src, (None, 0));

    while let Some(e) = queue.pop() {
        for edge in graph.edges(e.node_id) {
            if edge.dst == src {
                continue;
            }
            let new_cost = e.cost + edge.cost;

            // NOTE: the ideal solution would be to update the cost of the existing nodes in the priority queue
            // using decrease_key. However, Rust's BinaryHeap does not support this operation.
            // We accept the duplication of nodes in the priority queue as a tradeoff
            if prevs
                .get(&edge.dst)
                .is_none_or(|(_, prev_cost)| new_cost < *prev_cost)
            {
                prevs.insert(edge.dst, (Some(e.node_id), new_cost));
                queue.push(QueueEntry {
                    cost: new_cost,
                    node_id: edge.dst,
                });
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
        assert_eq!(dijkstra(&graph, 0), dists_a);

        let mut dists_b = BTreeMap::new();
        dists_b.insert(1, (None, 0));
        dists_b.insert(0, (Some(1), 10));
        dists_b.insert(2, (Some(0), 22));
        dists_b.insert(3, (Some(2), 54));
        assert_eq!(dijkstra(&graph, 1), dists_b);

        let mut dists_c = BTreeMap::new();
        dists_c.insert(2, (None, 0));
        dists_c.insert(1, (Some(2), 20));
        dists_c.insert(3, (Some(2), 32));
        dists_c.insert(0, (Some(1), 30));
        assert_eq!(dijkstra(&graph, 2), dists_c);

        let mut dists_d = BTreeMap::new();
        dists_d.insert(3, (None, 0));
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

    #[test]
    fn graph3() {
        let mut graph = Graph::default();
        let rows = 100;
        let cols = 100;
        let nid = |r: usize, c: usize| r * cols + c;

        for row in 0..rows {
            for col in 0..cols {
                graph.nodes.push((row, col));
                if row + 1 < rows {
                    graph.add_edge(nid(row, col), nid(row + 1, col), 1);
                }
                if col + 1 < cols {
                    graph.add_edge(nid(row, col), nid(row, col + 1), 1);
                }
                if row + 1 < rows && col + 1 < cols {
                    graph.add_edge(nid(row, col), nid(row + 1, col + 1), 1);
                }
            }
        }

        let res = dijkstra(&graph, 0);
        for row in 0..rows {
            for col in 0..cols {
                let expected_cost = row.max(col);
                let id = nid(row, col);
                assert_eq!(res[&id].1, expected_cost);
            }
        }
    }
}
