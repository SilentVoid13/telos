pub use std::fmt::Debug;

pub trait HeapVal: Ord + Debug {}
impl<T: Ord + Debug> HeapVal for T {}

pub type NodeIdx = usize;

/// Binary min-heap implementation
#[derive(Debug, Default)]
pub struct BinaryHeap<T: HeapVal> {
    pub nodes: Vec<T>,
}

impl<T: HeapVal> BinaryHeap<T> {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn from_vec(nodes: Vec<T>) -> Self {
        let mut heap = Self::new();
        heap.nodes = nodes;
        for i in (0..heap.nodes.len()).rev() {
            heap.heapify_down(i);
        }
        heap
    }

    pub fn push(&mut self, val: T) {
        self.nodes.push(val);
        self.heapify_up(self.nodes.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.nodes.is_empty() {
            return None;
        }
        // pop root, the left child automatically becomes the new root
        let popped = self.nodes.remove(0);
        if !self.nodes.is_empty() {
            self.heapify_down(0);
        }
        Some(popped)
    }

    #[inline]
    pub fn parent(&self, ni: NodeIdx) -> NodeIdx {
        (ni - 1) / 2
    }

    #[inline]
    pub fn children(&self, ni: NodeIdx) -> (Option<NodeIdx>, Option<NodeIdx>) {
        let left = ni * 2 + 1;
        let right = ni * 2 + 2;
        (
            if left < self.nodes.len() {
                Some(left)
            } else {
                None
            },
            if right < self.nodes.len() {
                Some(right)
            } else {
                None
            },
        )
    }

    pub fn heapify_up(&mut self, mut ni: NodeIdx) {
        // while we have a parent and we are larger than him
        while ni != 0 && self.nodes[self.parent(ni)] > self.nodes[ni] {
            let parent_id = self.parent(ni);
            self.nodes.swap(ni, parent_id);
            ni = parent_id;
        }
    }

    pub fn heapify_down(&mut self, mut ni: NodeIdx) {
        loop {
            let (left, right) = self.children(ni);
            let mut largest = ni;

            // for a min-heap we swap with the largest child

            // if the left child is larger than the current node
            if let Some(left) = left {
                if self.nodes[left] < self.nodes[largest] {
                    largest = left;
                }
            }
            // if the right child is larger than the current node
            if let Some(right) = right {
                if self.nodes[right] < self.nodes[largest] {
                    largest = right;
                }
            }

            // we are at the correct position
            if largest == ni {
                break;
            }
            self.nodes.swap(ni, largest);
            ni = largest;
        }
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.nodes.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_heap() {
        let mut heap: BinaryHeap<usize> = BinaryHeap::new();
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = BinaryHeap::new();
        heap.push(4);
        heap.push(2);
        heap.push(9);
        heap.push(11);
        dbg!(&heap);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(9));
        heap.push(1);
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(11));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_iter_heap() {
        let mut heap = BinaryHeap::new();
        heap.push(4);
        heap.push(2);
        heap.push(9);
        heap.push(11);

        let mut iter = heap.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&9));
        assert_eq!(iter.next(), Some(&11));
        assert_eq!(iter.next(), None);

        assert_eq!(heap.len(), 4);
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(9));
        assert_eq!(heap.pop(), Some(11));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_from_vec_min() {
        let vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        let mut heap = BinaryHeap::from_vec(vec);
        assert_eq!(heap.len(), 9);
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        heap.push(0);
        assert_eq!(heap.pop(), Some(0));
    }
}
