use crate::linked_list::{iter::{Iter, IterMut}, linked_list::LinkedList};

pub trait LinkedListTrait<T>{
    fn new()->LinkedList<T>;
    fn from_vec(values: Vec<T>)->LinkedList<T>;
    fn push(&mut self, value: T);
    fn insert(&mut self, value: T, index: usize)-> Result<(), String>;
    fn remove(&mut self, index: usize) -> Result<(), String>;
    fn get(&self, index: usize) -> Result<&T, String>;
    fn get_mut(&mut self, index: usize) -> Result<&mut T, String>;
    fn contains(&self, value:&T)->bool;
    fn size(&self) -> usize;
    fn iter(&self) -> Iter<'_, T>;
    fn iter_mut(&mut self) -> IterMut<'_, T>;
}