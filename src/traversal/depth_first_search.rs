use std::collections::{HashSet, VecDeque};

pub struct Node(usize);
pub type NodeId = usize;

pub type Edge = (NodeId, NodeId);

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn neighbours(&self, node: NodeId) -> Vec<NodeId> {
        self.edges
            .iter()
            .filter(|(from, _)| *from == node)
            .map(|(_, to)| *to)
            .collect()
    }
}

/// Naive Recursive Depth First Search
/// This doesn't support bidirectional nodes (infinite recursion)
/// It is best suited for traversing tree-like structures
pub fn depth_first_search_rec(
    graph: &Graph,
    path: &mut Vec<NodeId>,
    cur_node_id: NodeId,
    target: &Node,
) -> Option<NodeId> {
    let cur_node = &graph.nodes[cur_node_id];
    path.push(cur_node.0);
    if cur_node.0 == target.0 {
        return Some(cur_node_id);
    }

    for neighbour in graph.neighbours(cur_node_id) {
        if let Some(v) = depth_first_search_rec(graph, path, neighbour, target) {
            return Some(v);
        }
    }
    None
}

/// Iterative Depth First Search
pub fn depth_first_search_it(
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

        for neighbour in graph.neighbours(node_id).into_iter().rev() {
            if visited.insert(neighbour) {
                queue.push_front(neighbour);
            }
        }
    }
    (path, None)
}

#[cfg(test)]
mod tests {
    use super::{depth_first_search_it, depth_first_search_rec, Graph, Node};

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
    fn dfs_rec_fail() {
        let graph = graph1();

        let root_id = 0;
        let target = Node(10);
        let correct_path = vec![1, 2, 4, 5, 8, 3, 6, 7];

        let mut path = Vec::new();
        let res = depth_first_search_rec(&graph, &mut path, root_id, &target);
        assert_eq!(res, None);
        assert_eq!(correct_path, path);
    }

    #[test]
    fn dfs_rec_success() {
        let graph = graph1();

        let root_id = 0;
        let target = Node(6);
        let correct_path = vec![1, 2, 4, 5, 8, 3, 6];

        let mut path = Vec::new();
        let res = depth_first_search_rec(&graph, &mut path, root_id, &target);
        assert_eq!(res, Some(5));
        assert_eq!(correct_path, path);
    }

    #[test]
    fn dfs_it_success() {
        let graph = graph2();

        let root_id = 0;
        let target = Node(4);
        let correct_path = vec![1, 2, 5, 6, 3, 4];

        let (path, res) = depth_first_search_it(&graph, root_id, &target);
        assert_eq!(correct_path, path);
        assert_eq!(res, Some(3));
    }

    #[test]
    fn dfs_it_fail() {
        let graph = graph2();

        let root_id = 0;
        let target = Node(8);
        let correct_path = vec![1, 2, 5, 6, 3, 4, 7];

        let (path, res) = depth_first_search_it(&graph, root_id, &target);
        assert_eq!(correct_path, path);
        assert_eq!(res, None);
    }
}
