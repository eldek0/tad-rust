use std::fmt::{Debug};

pub struct Node<T>{
    pub value:T,
    pub next:Option<Box<Node<T>>>
}

impl <T> Node<T>{
    pub fn new(value:T) -> Node<T>{
        return Node{value, next:None};
    }
}

impl <T:PartialEq> PartialEq for Node<T>{
    fn eq(&self, other: &Self) -> bool {
        return self.value == other.value;
    }

    fn ne(&self, other: &Self) -> bool {
        return self.value != other.value;
    }
}

impl <T:Debug> Debug for Node<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Node({:?})", self.value);
    }
}