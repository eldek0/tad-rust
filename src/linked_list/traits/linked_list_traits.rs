use crate::linked_list::{iter::{Iter, IterMut}, linked_list::LinkedList, linked_list_error::LinkedListError};

pub trait LinkedListTrait<T>{
    fn new()->LinkedList<T>;
    fn from_vec(values: Vec<T>)->LinkedList<T>;
    fn push(&mut self, value: T);
    fn insert(&mut self, value: T, index: usize)-> Result<(), LinkedListError>;
    fn remove(&mut self, index: usize) -> Result<T, LinkedListError>;
    fn get(&self, index: usize) -> Result<&T, LinkedListError>;
    fn get_mut(&mut self, index: usize) -> Result<&mut T, LinkedListError>;
    fn contains(&self, value:&T)->bool;
    fn size(&self) -> usize;
    fn iter(&self) -> Iter<'_, T>;
    fn iter_mut(&mut self) -> IterMut<'_, T>;
    fn is_empty(&self)->bool;
}