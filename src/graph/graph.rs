use std::{fmt::Debug, hash::Hash};

use crate::{graph::traits::graph_traits::GraphTrait, hash_map::{hash_map::HashMap, traits::hash_map_traits::HashMapTrait}, linked_list::{linked_list::LinkedList, traits::linked_list_traits::LinkedListTrait}};

pub struct Graph<T:Hash+Clone>{
    map:HashMap<T, HashMap<T, i32>>,
}

impl <T:Hash+Debug+Eq+Clone> GraphTrait<T> for Graph<T>  {
    fn new()->Self {
        return Graph { map: HashMap::new(1024) }
    }

    fn add_vertex(&mut self, vertex:T) {
        self.map.put(vertex, HashMap::new(1024)).ok();
    }

    fn add_edge(&mut self, source:T, destination:T, bidireccional:bool, weight:Option<i32>) {
        let w = weight.unwrap_or(0);

        if !self.map.contains_key(&source){
            self.add_vertex(source.clone());
        }

        if !self.map.contains_key(&destination){
            self.add_vertex(destination.clone());
        }

        self.map.get_mut(&source).unwrap().put(destination.clone(), w).ok();
        if bidireccional{
            self.map.get_mut(&destination).unwrap().put(source, w).ok();
        }
    }

    fn has_vertex(&self, vertex:&T)->bool {
        return self.map.contains_key(vertex);
    }

    fn has_edge(&self, vertex1:&T, vertex2:&T)->bool {
        if self.has_vertex(vertex1){
            return self.map.get(vertex1).unwrap().contains_key(vertex2);
        }
        return false;
    }

    fn get_neighbors(&self, vertex:&T)->LinkedList<T> {
        let mut neighbors = LinkedList::new();
        if !self.has_vertex(vertex){
            return neighbors;
        }

        for (k, v) in self.map.get(vertex).unwrap().iter() {
            neighbors.push(k.clone());
        }

        neighbors
    }

    fn vertex_count(&self)->usize {
        return self.map.size();
    }

    fn edge_count(&self)->usize {
        let mut count = 0;
        for (k, v) in self.map.iter(){
            count += v.size();
        }

        return count;
    }
}

/// Debug print
impl <T:Hash+Debug+Clone+PartialEq> Debug for Graph<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        for (k1, v1) in self.map.iter(){
            write!(f, "\t{:?} -> ", k1)?;
            write!(f, "[")?;
            let mut first = true;
            for (k2, _) in v1.iter() {
                if !first {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", k2)?;
                first = false;
            }
            writeln!(f, "]")?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}