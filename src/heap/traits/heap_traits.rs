use crate::heap::heap_error::HeapError;

pub trait HeapTrait<K: PartialOrd, T> {
    fn new(min:bool, capacity:usize) -> Self;
    fn push(&mut self, key: K, val: T);
    fn pop(&mut self)-> Result<(K, T), HeapError>;
    fn peek(&self) -> Result<(&K, &T), HeapError>;
    fn peek_mut(&mut self) -> Result<(&K, &mut T), HeapError>;
    fn size(&self) -> usize;
    fn is_empty(&self)->bool;
}