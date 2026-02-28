use std::{fmt::{Debug, Formatter}, hash::Hash};

use crate::hash_map::{hash_map::HashMap, traits::hash_map_traits::HashMapTrait};

pub struct Node<T>{
    pub children:HashMap<T, Node<T>>,
    pub is_end:bool
}

impl <T:Hash+Debug+PartialEq> Node<T>{
    pub fn new() -> Self {
        Node {
            children: HashMap::new(1024),
            is_end: false,
        }
    }
}

impl <T:Debug+Hash+PartialEq> Debug for Node<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_end{
            write!(f, "end")?;
            return Ok(());
        }
        write!(f, "{:?}", self.children)?;
        Ok(())
    }
    
}