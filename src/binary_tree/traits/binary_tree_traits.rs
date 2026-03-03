use crate::binary_tree::binary_tree_error::BinaryTreeError;

pub trait BinaryTreeTrait<K, T>{
    fn new() -> Self;
    fn insert(&mut self, key: K, value: T, parent:Option<K>)->Result<(), BinaryTreeError<K>>;
    fn delete(&mut self, key: &K)->Result<(), BinaryTreeError<K>>;
    fn find(&self, key: &K) -> Result<(&K, &T), BinaryTreeError<K>>;
    fn find_mut(&mut self, key: &K) -> Result<(&K, &mut T), BinaryTreeError<K>>;
    fn size(&self)->usize;
    fn count_leaves(&self)->usize;
    fn is_empty(&self)->bool;
}