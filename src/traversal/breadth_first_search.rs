use std::collections::{HashSet, VecDeque};

pub struct Node(usize);
pub type NodeId = usize;

pub type Edge = (NodeId, NodeId);

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn neighbours(&self, node_id: NodeId) -> Vec<NodeId> {
        self.edges
            .iter()
            .filter(|(from, _)| *from == node_id)
            .map(|(_, to)| *to)
            .collect()
    }
}

/// Breadth First Search
pub fn breadth_first_search(
    graph: &Graph,
    root_id: NodeId,
    target: &Node,
) -> (Vec<NodeId>, Option<NodeId>) {
    let mut path = Vec::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(root_id);
    visited.insert(root_id);

    while let Some(node_id) = queue.pop_front() {
        let node = &graph.nodes[node_id];
        path.push(node.0);
        if node.0 == target.0 {
            return (path, Some(node_id));
        }

        for neighbour_id in graph.neighbours(node_id) {
            if visited.insert(neighbour_id) {
                queue.push_back(neighbour_id);
            }
        }
    }
    (path, None)
}

#[cfg(test)]
mod tests {
    use super::{breadth_first_search, Graph, Node};

    /* Example graph #1:
     *
     *            (1)   <--- Root
     *           /   \
     *         (2)   (3)
     *        / |     | \
     *     (4) (5)   (6) (7)
     *          |
     *         (8)
     */
    fn graph1() -> Graph {
        let nodes = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let edges = vec![(0, 1), (0, 2), (1, 3), (1, 4), (2, 5), (2, 6), (4, 7)];

        Graph {
            nodes: nodes.into_iter().map(Node).collect(),
            edges,
        }
    }

    /* Example graph #2:
     *
     *     (1) --- (2)     (3) --- (4)
     *            / |     /       /
     *          /   |   /       /
     *        /     | /       /
     *     (5)     (6) --- (7)     (8)
     */
    fn graph2() -> Graph {
        let nodes = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let edges = vec![
            (0, 1),
            (1, 0),
            (1, 4),
            (4, 1),
            (1, 5),
            (5, 1),
            (2, 3),
            (3, 2),
            (2, 5),
            (5, 2),
            (3, 6),
            (6, 3),
            (5, 6),
            (6, 5),
        ];

        Graph {
            nodes: nodes.into_iter().map(Node).collect(),
            edges,
        }
    }

    #[test]
    fn bfs_fail1() {
        let graph = graph1();
        let root_id = 0;
        let target = Node(10);

        let (_, found) = breadth_first_search(&graph, root_id, &target);
        assert_eq!(found, None);
    }

    #[test]
    fn bfs_success1() {
        let graph = graph1();

        let root_id = 0;
        let target = Node(8);
        let expected_path = vec![1, 2, 3, 4, 5, 6, 7, 8];

        let (path, found) = breadth_first_search(&graph, root_id, &target);

        assert_eq!(found, Some(7));
        assert_eq!(path, expected_path);
    }

    #[test]
    fn bfs_fail2() {
        let graph = graph2();

        let root_id = 0;
        let target = Node(8);

        let (_, found) = breadth_first_search(&graph, root_id, &target);
        assert_eq!(found, None);
    }

    #[test]
    fn bfs_success2() {
        let graph = graph2();

        let root_id = 3;
        let target = Node(1);
        let expected_path = vec![4, 3, 7, 6, 2, 1];

        let (path, found) = breadth_first_search(&graph, root_id, &target);
        assert_eq!(found, Some(0));
        assert_eq!(path, expected_path);
    }
}
