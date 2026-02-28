use crate::linked_list::linked_list::LinkedList;


pub trait PrefixTreeTrait<T>{
    fn new()->Self;
    fn insert(&mut self, sequence: impl IntoIterator<Item = T>);
    fn insert_string(&mut self, sequence: &str) where T:From<char>;
    fn remove(&mut self, sequence: impl IntoIterator<Item = T>);
    fn remove_string(&mut self, sequence: &str) where T:From<char>;
    fn search(&self, sequence: impl IntoIterator<Item = T>)->bool;
    fn search_string(&self, sequence: &str)->bool where T:From<char>;
    fn autocomplete(&self, prefix: impl IntoIterator<Item = T>) -> LinkedList<LinkedList<T>>;
    fn autocomplete_string(&self, prefix: &str) -> LinkedList<String> where T:From<char>+Into<char>;
    fn size(&self)->usize;
    fn is_empty(&self)->bool;
}