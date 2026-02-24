pub trait BinaryTreeTrait<K, T>{
    fn new() -> Self;
    fn insert(&mut self, key: K, value: T, parent:Option<K>)->Result<(),String>;
    fn delete(&mut self, key: K)->Result<(K, T),String>;
    fn find(&self, key: K) -> Result<(&K, &T), String>;
    fn size(&self)->usize;
    fn count_leaves(&self)->usize;
}