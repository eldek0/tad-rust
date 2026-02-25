use std::collections::VecDeque;

use crate::hash_map::{node::Node, traits::hash_map_traits::HashMapTrait};

pub struct HashMap<K, T>{
    hash: VecDeque<Node<K, T>>,
    size: usize,
    capacity: usize,
    load_factor:f32
}

impl <K, T> HashMapTrait<K, T> for HashMap<K, T>{
    fn put(&mut self, key:K, value:T)->Result<(), String> {
        todo!()
    }

    fn get(&self, key:K)->Result<(&K, &T), String> {
        todo!()
    }

    fn get_mut(&mut self, key:K)->Result<(&K, &mut T), String> {
        todo!()
    }

    fn contains(&self, key:K)->bool {
        todo!()
    }

    fn remove(&mut self, key:K)->Result<(), String> {
        todo!()
    }
}