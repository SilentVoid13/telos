use std::ptr::NonNull;

pub struct Node {
    pub val: i32,
    pub next: NodePtr,
}

pub type NodePtr = Option<NonNull<Node>>;

#[derive(Default)]
pub struct LinkedList {
    pub head: NodePtr,
    pub tail: NodePtr,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList::default()
    }

    pub fn push(&mut self, val: i32) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node { val, next: None })));
            if let Some(old) = self.tail {
                (*old.as_ptr()).next = Some(new);
            } else {
                self.head = Some(new);
            }
            self.tail = Some(new);
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        unsafe {
            if let Some(head) = self.head.take() {
                let head = Box::from_raw(head.as_ptr());
                self.head = head.next;
                if self.head.is_none() {
                    self.tail = None;
                }
                return Some(head.val);
            }
            None
        }
    }

    pub fn peek(&self) -> Option<&i32> {
        unsafe { self.head.map(|n| &(*n.as_ptr()).val) }
    }

    pub fn peek_mut(&mut self) -> Option<&mut i32> {
        unsafe { self.head.map(|n| &mut (*n.as_ptr()).val) }
    }

    pub fn iter(&self) -> LinkedListIter<'_> {
        LinkedListIter { next: &self.head }
    }

    pub fn iter_mut(&mut self) -> LinkedListIterMut<'_> {
        LinkedListIterMut {
            next: &mut self.head,
        }
    }
}

impl Drop for LinkedList {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

pub struct LinkedListIntoIter(LinkedList);
impl IntoIterator for LinkedList {
    type Item = i32;
    type IntoIter = LinkedListIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIntoIter(self)
    }
}
impl Iterator for LinkedListIntoIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct LinkedListIter<'a> {
    next: &'a NodePtr,
}
impl<'a> Iterator for LinkedListIter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| unsafe {
            self.next = &(*node.as_ptr()).next;
            &(*node.as_ptr()).val
        })
    }
}

pub struct LinkedListIterMut<'a> {
    next: &'a NodePtr,
}
impl<'a> Iterator for LinkedListIterMut<'a> {
    type Item = &'a mut i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| unsafe {
            self.next = &(*node.as_ptr()).next;
            &mut (*node.as_ptr()).val
        })
    }
}

#[cfg(test)]
mod test {
    use super::LinkedList;
    #[test]
    fn basics() {
        let mut list = LinkedList::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        list.push(6);
        list.push(7);
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn miri_test() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert!(list.pop() == Some(1));
        list.push(4);
        assert!(list.pop() == Some(2));
        list.push(5);

        assert!(list.peek() == Some(&3));
        list.push(6);
        if let Some(x) = list.peek_mut() {
            *x *= 10;
        }
        assert!(list.peek() == Some(&30));
        assert!(list.pop() == Some(30));

        for elem in list.iter_mut() {
            *elem *= 100;
        }
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&400));
        assert_eq!(iter.next(), Some(&500));
        assert_eq!(iter.next(), Some(&600));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        assert!(list.pop() == Some(400));
        if let Some(x) = list.peek_mut() {
            *x *= 10;
        }
        assert!(list.peek() == Some(&5000));
        list.push(7);
    }
}
