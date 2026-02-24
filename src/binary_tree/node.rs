use std::fmt::{self, Debug};
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

    fn find(&self, key: K) -> Option<&Node<K, T>> {
        if self.key == key {
            return Some(self);
        }

        if let Some(left) = self.left.as_ref(){
            let found = left.find(key.clone());
            if found.is_some(){
                return found;
            }
        }

        if let Some(right) = self.right.as_ref(){
            let found = right.find(key.clone());
            if found.is_some(){
                return found;
            }
        }

        return None;
    }

    fn find_mut(&mut self, key: K) -> Option<&mut Node<K, T>> {
        if self.key == key {
            return Some(self);
        }

        if let Some(left) = self.left.as_mut(){
            let found = left.find_mut(key.clone());
            if found.is_some(){
                return found;
            }
        }

        if let Some(right) = self.right.as_mut(){
            let found = right.find_mut(key.clone());
            if found.is_some(){
                return found;
            }
        }

        return None;
    }

    fn find_parent(&mut self, key: K) -> Option<&mut Node<K, T>>{
        if let Some(left) = self.left.as_ref() {
            if left.key == key {
                return Some(self);
            }
        }

        if let Some(right) = self.right.as_ref() {
            if right.key == key {
                return Some(self);
            }
        }

        if let Some(left) = self.left.as_mut() {
            if let Some(parent) = left.find_parent(key.clone()) {
                return Some(parent);
            }
        }

        if let Some(right) = self.right.as_mut() {
            return right.find_parent(key);
        }

        return None;
    }
}

impl <K: Debug + PartialEq, T:Debug> Debug for Node<K, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_level(f, 0)
    }
}

impl<K: Debug + PartialEq, T: Debug> Node<K,T> {
    fn fmt_with_level(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        for _ in 0..level {
            write!(f, "\t")?;
        }

        writeln!(f, "\t({:?}, {:?})", self.key, self.value)?;

        if let Some(left) = &self.left {
            left.fmt_with_level(f, level + 1)?;
        }

        if let Some(right) = &self.right {
            right.fmt_with_level(f, level + 1)?;
        }

        Ok(())
    }
}