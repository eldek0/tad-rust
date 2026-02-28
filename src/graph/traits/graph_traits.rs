use std::{hash::Hash};
use crate::linked_list::linked_list::LinkedList;

pub trait GraphTrait<T:Hash+Eq+Clone> {
    fn new()->Self;
    fn add_vertex(&mut self, vertex:T);
    fn add_edge(&mut self, source:T, destination:T, bidireccional:bool, weight:Option<i32>);
    fn has_vertex(&self, vertex:&T)->bool;
    fn has_edge(&self, vertex1:&T, vertex2:&T)->bool;
    fn get_neighbors(&self, vertex:&T)->LinkedList<T>;
    fn vertex_count(&self)->usize;
    fn edge_count(&self)->usize;
    fn is_empty(&self)->bool;
}