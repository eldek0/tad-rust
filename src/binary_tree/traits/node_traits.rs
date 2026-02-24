use crate::binary_tree::node::Node;

pub trait NodeTrait<K: PartialEq, T> {
    fn new(key: K, value: T) -> Self;
    fn find(&self, key: K) -> Option<&Node<K, T>>;
    fn find_mut(&mut self, key: K) -> Option<&mut Node<K, T>>;
    fn find_parent(&mut self, key: K) -> Option<&mut Node<K, T>>;
}