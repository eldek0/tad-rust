pub trait HeapTrait<K: PartialOrd, T> {
    fn new(min:bool, capacity:usize) -> Self;
    fn push(&mut self, key: K, val: T);
    fn pop(&mut self)-> Result<(K, T), String>;
    fn peek(&self) -> Result<(&K, &T), String>;
    fn peek_mut(&mut self) -> Result<(&K, &mut T), String>;
    fn size(&self) -> usize;
    fn is_empty(&self)->bool;
}