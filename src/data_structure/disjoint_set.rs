pub struct DisjointSet {
    parent: Vec<usize>,
    // a rank is an upper bound on the height of a tree
    // rank is used to keep the tree flat during union
    rank: Vec<usize>,
}

impl DisjointSet {
    pub fn new(size: usize) -> Self {
        let parent = (0..size).collect();
        let rank = vec![0; size];
        Self { parent, rank }
    }

    pub fn find(&mut self, mut x: usize) -> usize {
        // path halving: a find operation presents an important opportunity for improving the forest
        // we want to reach the root node as quickly as possible
        // during our ascent to the root node, we make each node in the path point to its parent
        // to improve future find operations

        // go up the parent pointer tree to search for root
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    // union by rank
    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            // the larger tree becomes the parent
            if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;
            } else if self.rank[root_x] < self.rank[root_y] {
                self.parent[root_x] = root_y;
            } else {
                // if ranks are the same, either one can be the parent
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let ds = DisjointSet::new(5);

        // Check initial parents
        assert_eq!(ds.parent, vec![0, 1, 2, 3, 4]);
        // Check initial ranks
        assert_eq!(ds.rank, vec![0; 5]);
    }

    #[test]
    fn test_find_singleton() {
        let mut ds = DisjointSet::new(5);
        // Each element should be its own parent initially
        for i in 0..5 {
            assert_eq!(ds.find(i), i);
        }
    }

    #[test]
    fn test_union() {
        let mut ds = DisjointSet::new(5);

        ds.union(0, 1);
        ds.union(1, 2);

        // 0, 1, 2 should belong to the same set
        assert_eq!(ds.find(0), ds.find(1));
        assert_eq!(ds.find(1), ds.find(2));

        // 3 and 4 should still belong to their own sets
        assert_ne!(ds.find(0), ds.find(3));
        assert_ne!(ds.find(0), ds.find(4));
    }

    #[test]
    fn test_union_by_rank() {
        let mut ds = DisjointSet::new(5);

        ds.union(0, 1); // 1 becomes the child of 0
        ds.union(2, 3); // 3 becomes the child of 2
        ds.union(0, 2); // Merge the sets of 0 and 2

        // Verify the parent structure and rank updates
        let root = ds.find(0);
        assert_eq!(ds.find(1), root);
        assert_eq!(ds.find(2), root);
        assert_eq!(ds.find(3), root);

        // 4 should still be its own set
        assert_eq!(ds.find(4), 4);
    }

    #[test]
    fn test_path_compression() {
        let mut ds = DisjointSet::new(5);

        ds.union(0, 1);
        ds.union(1, 2);
        ds.union(2, 3);

        // Before path compression, nodes may not directly point to the root
        let root = ds.find(0);

        // After find operations, path compression should occur
        for i in 0..4 {
            assert_eq!(ds.find(i), root);
            assert_eq!(ds.parent[i], root);
        }
    }

    #[test]
    fn test_disjoint_sets() {
        let mut ds = DisjointSet::new(6);

        ds.union(0, 1);
        ds.union(2, 3);
        ds.union(4, 5);

        // Check that disjoint sets are maintained
        assert_eq!(ds.find(0), ds.find(1));
        assert_eq!(ds.find(2), ds.find(3));
        assert_eq!(ds.find(4), ds.find(5));
    }

    #[test]
    fn test_large_union() {
        let mut ds = DisjointSet::new(100);

        // Union all elements into a single set
        for i in 0..99 {
            ds.union(i, i + 1);
        }

        let root = ds.find(0);
        // Ensure all elements belong to the same set
        for i in 1..100 {
            assert_eq!(ds.find(i), root);
        }
    }
}
