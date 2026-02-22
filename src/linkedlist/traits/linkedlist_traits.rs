use crate::linkedlist::linkedlist::Linkedlist;

pub trait LinkedlistTrait<T>{
    fn new()->Linkedlist<T>;
    fn new_from(values: Vec<T>)->Linkedlist<T>;
    fn add(&mut self, value: T);
    fn remove(&mut self, index: usize) -> Result<(), String>;
    fn get(&self, index: usize) -> Result<&T, String>;
    fn size(&self) -> usize;
}