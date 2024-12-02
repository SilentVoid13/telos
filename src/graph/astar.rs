use std::collections::{BTreeMap, BinaryHeap, HashMap};

pub trait NodeVal: Ord + Copy + Default {}
impl<T: Ord + Copy + Default> NodeVal for T {}

pub type NodeId = usize;

#[derive(Default)]
pub struct Graph<V: NodeVal> {
    pub nodes: Vec<V>,
    pub edges: HashMap<NodeId, Vec<Edge>>,
}

pub struct Edge {
    #[allow(dead_code)]
    src: NodeId,
    dst: NodeId,
    cost: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub struct QueueEntry {
    real_cost: usize,
    heur_cost: usize,
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
        other.heur_cost.cmp(&self.heur_cost)
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

pub fn astar<V: NodeVal>(
    graph: &Graph<V>,
    src: NodeId,
    target: V,
    heuristic: impl Fn(&Graph<V>, NodeId) -> usize,
) -> Option<(usize, Vec<NodeId>)> {
    let mut prevs = BTreeMap::new();
    let mut queue = BinaryHeap::new();

    queue.push(QueueEntry {
        real_cost: 0,
        heur_cost: heuristic(graph, src),
        node_id: src,
    });
    prevs.insert(src, (None, 0));

    let mut found = None;
    while let Some(e) = queue.pop() {
        let nval = graph.nodes[e.node_id];
        if nval == target {
            found = Some((e.node_id, e.real_cost));
            break;
        }
        for edge in graph.edges(e.node_id) {
            if edge.dst == src {
                continue;
            }
            let new_rcost = e.real_cost + edge.cost;

            // NOTE: the ideal solution would be to update the cost of the existing nodes in the priority queue
            // using decrease_key. However, Rust's BinaryHeap does not support this operation.
            // We accept the duplication of nodes in the priority queue as a tradeoff
            if prevs.get(&edge.dst).map_or(true, |(_, c)| new_rcost < *c) {
                let new_hcost = new_rcost + heuristic(graph, edge.dst);
                prevs.insert(edge.dst, (Some(e.node_id), new_rcost));
                queue.push(QueueEntry {
                    real_cost: new_rcost,
                    heur_cost: new_hcost,
                    node_id: edge.dst,
                });
            }
        }
    }

    // reconstruct path
    let (target_id, target_cost) = found?;
    let mut path = vec![target_id];
    let mut cur = target_id;
    while cur != src {
        let (prev_node, _) = prevs[&cur];
        cur = prev_node?;
        path.push(cur);
    }
    path.reverse();
    Some((target_cost, path))
}

#[cfg(test)]
mod tests {
    use super::{astar, Graph, NodeVal};

    fn null_heuristic<V: NodeVal>(_: &Graph<V>, _: usize) -> usize {
        0
    }

    // https://imagedelivery.net/CLfkmk9Wzy8_9HRyug4EVA/482027d5-fb4e-4a3c-d710-ec60cbead600/sharpen=1
    #[test]
    fn test_dijkstra() {
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

        assert_eq!(
            astar(&graph, 0, 1, null_heuristic),
            Some((7, vec![0, 2, 1]))
        );
        assert_eq!(astar(&graph, 0, 2, null_heuristic), Some((3, vec![0, 2])));
        assert_eq!(
            astar(&graph, 0, 3, null_heuristic),
            Some((9, vec![0, 2, 1, 3]))
        );
        assert_eq!(
            astar(&graph, 0, 4, null_heuristic),
            Some((5, vec![0, 2, 4]))
        );
    }

    #[test]
    fn test_heuristic() {
        // make a grid
        let mut graph = Graph::default();
        let rows = 500;
        let cols = 500;

        let nid = |r, c| r * cols + c;
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

        let target = (455usize, 479usize);

        // manhattan distance
        let h = |g: &Graph<(usize, usize)>, nid: usize| {
            let (r, c) = g.nodes[nid];
            let m = (target.0 as i32 - r as i32).abs() + (target.1 as i32 - c as i32).abs();
            m as usize
        };

        // Dijkstra would explore most of the nodes
        // the heuristic should allow exploring far fewer nodes
        let now = std::time::Instant::now();
        let res = astar(&graph, 0, target, h).unwrap();
        assert!(now.elapsed() < std::time::Duration::from_millis(10));
        assert_eq!(res.0, 479);
        assert_eq!(res.1.len(), 480);
    }
}
