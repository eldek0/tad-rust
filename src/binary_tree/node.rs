use std::fmt::Debug;
use crate::binary_tree::traits::node_traits::NodeTrait;

pub struct Node<K:PartialEq, T> {
    pub key: K,
    pub value: T,
    pub left: Option<Box<Node<K, T>>>,
    pub right: Option<Box<Node<K, T>>>,
}

impl<K:PartialEq + Clone, T> NodeTrait<K, T> for Node<K, T> {
    fn new(key: K, value: T) -> Node<K, T> {
        Node { key, value, left: None, right: None }
    }

    fn find(&mut self, key: K) -> Option<&mut Node<K, T>> {
        if self.key == key {
            return Some(self);
        }

        if let Some(left) = self.left.as_mut(){
            let found = left.find(key.clone());
            if found.is_some(){
                return found;
            }
        }

        if let Some(right) = self.right.as_mut(){
            let found = right.find(key.clone());
            if found.is_some(){
                return found;
            }
        }

        return None;
    }

    fn find_parent(&self, key: K) -> Option<&Node<K, T>> {
        if self.key == key {
            return Some(self);
        }

        if self.left.is_some(){
            let left = self.left.as_ref().unwrap();
            let found = left.find_parent(key.clone());
            if found.is_some(){
                return found;
            }
        }

        if self.right.is_some() {
            let right = self.right.as_ref().unwrap();
            let found = right.find_parent(key);
            if found.is_some() {
                return found;
            }
        }
        return None;
    }
}

impl <K:Debug + PartialEq, T:Debug> Debug for Node<K, T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.key, self.value)
    }
}