use std::cmp::Ordering;
use std::fmt::{Debug, Display};

pub struct Node<K, T> {
    pub key:K,
    pub value:T,
}

impl <K, T> Node<K, T> {
    pub fn new(key: K, value: T) -> Node<K, T> {
        Node { key, value }
    }
}

impl <K:PartialEq, T> PartialEq for Node<K, T>{
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
    fn ne(&self, other: &Self) -> bool {
        self.key != other.key
    }
}

impl <K:PartialOrd, T> PartialOrd for Node<K, T>{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.key < other.key {
            return Some(Ordering::Less);
        }
        if self.key > other.key {
            return Some(Ordering::Greater);
        }

        Some(Ordering::Equal)

    }
}

impl <K:Debug, T:Debug> Debug for Node<K, T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.key, self.value)
    }
}