use crate::linkedlist::linked_list::Linkedlist;

pub trait LinkedlistTrait<T>{
    fn new()->Linkedlist<T>;
    fn add(&mut self, value: T);
    fn remove(&mut self, index: usize);
    fn get(&self, index: usize) -> Result<&T, String>;
    fn size(&self) -> usize;
}