use std::collections::{HashSet, VecDeque};

pub struct Node(usize);
pub type NodeId = usize;

pub type Edge = (NodeId, NodeId);

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

/// Naive Recursive Depth First Search
/// This doesn't support bidirectional nodes (infinite recursion)
/// It is best suited for traversing structures like binary trees
pub fn depth_first_search_rec(
    graph: &Graph,
    path: &mut Vec<NodeId>,
    cur_node_id: NodeId,
    target: &Node,
) -> Option<NodeId> {
    if !path.contains(&cur_node_id) {
        path.push(cur_node_id);
    }

    let root = &graph.nodes[cur_node_id];
    if root.0 == target.0 {
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

    while let Some(node_id) = queue.pop_front() {
        path.push(node_id);
        if node_id == target.0 {
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

impl Graph {
    pub fn neighbours(&self, node: NodeId) -> Vec<NodeId> {
        self.edges
            .iter()
            .filter(|(from, _)| *from == node)
            .map(|(_, to)| *to)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{depth_first_search_it, depth_first_search_rec, Graph, Node};

    #[test]
    fn find_rec_fail() {
        let nodes = vec![1, 2, 3, 4, 5, 6, 7];
        let edges = vec![(0, 1), (0, 2), (1, 3), (1, 4), (2, 5), (2, 6)];

        let root_id = 0;
        let target = Node(99);
        let correct_path = vec![0, 1, 3, 4, 2, 5, 6];

        let graph = Graph {
            nodes: nodes.into_iter().map(Node).collect(),
            edges,
        };

        let mut path = Vec::new();
        let res = depth_first_search_rec(&graph, &mut path, root_id, &target);
        assert_eq!(res, None);
        assert_eq!(correct_path, path);
    }

    #[test]
    fn find_rec_success() {
        let nodes = vec![1, 2, 3, 4, 5, 6, 7];
        let edges = vec![(0, 1), (0, 2), (1, 3), (1, 4), (2, 5), (2, 6)];

        let root_id = 0;
        let target = Node(6);
        let correct_path = vec![0, 1, 3, 4, 2, 5];

        let graph = Graph {
            nodes: nodes.into_iter().map(Node).collect(),
            edges,
        };

        let mut path = Vec::new();
        let res = depth_first_search_rec(&graph, &mut path, root_id, &target);
        assert_eq!(res, Some(5));
        assert_eq!(correct_path, path);
    }

    #[test]
    fn find_it_success() {
        let nodes = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let edges = vec![
            (0, 1),
            (1, 3),
            (3, 2),
            (2, 1),
            (3, 4),
            (4, 5),
            (5, 7),
            (7, 6),
            (6, 4),
        ];

        let root_id = 0;
        let target = Node(6);
        let correct_path = vec![0, 1, 3, 2, 4, 5, 7, 6];

        let graph = Graph {
            nodes: nodes.into_iter().map(Node).collect(),
            edges,
        };
        let (path, res) = depth_first_search_it(&graph, root_id, &target);
        assert_eq!(correct_path, path);
        assert_eq!(res, Some(6));
    }

    #[test]
    fn find_it_success_2() {
        let nodes = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let edges = vec![
            (0, 1),
            (1, 3),
            (3, 2),
            (2, 1),
            (3, 4),
            (4, 5),
            (5, 7),
            (7, 6),
            (6, 4),
        ];

        let root_id = 0;
        let target = Node(4);

        let correct_path = vec![0, 1, 3, 2, 4];

        let graph = Graph {
            nodes: nodes.into_iter().map(Node).collect(),
            edges,
        };
        let (path, res) = depth_first_search_it(&graph, root_id, &target);
        assert_eq!(correct_path, path);
        assert_eq!(res, Some(4));
    }
}
