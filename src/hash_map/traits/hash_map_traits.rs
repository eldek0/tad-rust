use crate::hash_map::hash_map_error::HashMapError;

pub trait HashMapTrait<K, T> {
    fn new(capacity:usize)->Self;
    fn insert(&mut self, key:K, value:T);
    fn get(&self, key:&K)->Result<&T, HashMapError<K>>;
    fn get_mut(&mut self, key:&K)->Result<&mut T, HashMapError<K>>;
    fn contains_key(&self, key:&K)->bool;
    fn remove(&mut self, key:&K)->Result<(), HashMapError<K>>;
    fn size(&self)->usize;
    fn capacity(&self)->usize;
    fn iter(&self) -> Box<dyn Iterator<Item = (&K, &T)> + '_>;
    fn is_empty(&self)->bool;
}