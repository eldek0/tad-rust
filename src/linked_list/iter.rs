use crate::linked_list::node::Node;

/// Iter
pub struct Iter<'a, T> {
    pub current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            self.current = node.next.as_deref();
            &node.value
        })
    }
}

/// Iter mut
pub struct IterMut<'a, T> {
    pub current: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            self.current = node.next.as_deref_mut();
            &mut node.value
        })
    }
}

/// Into iter
pub struct IntoIter<T> {
    pub current: Option<Box<Node<T>>>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|boxed_node| {
            let node = *boxed_node;
            self.current = node.next;
            node.value
        })
    }
}