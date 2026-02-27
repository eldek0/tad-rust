pub trait HashMapTrait<K, T> {
    fn new(capacity:usize)->Self;
    fn put(&mut self, key:K, value:T)->Result<(), String>;
    fn get(&self, key:&K)->Result<&T, String>;
    fn get_mut(&mut self, key:&K)->Result<&mut T, String>;
    fn contains_key(&self, key:&K)->bool;
    fn remove(&mut self, key:&K)->Result<(), String>;
    fn size(&self)->usize;
    fn capacity(&self)->usize;
    fn iter(&self) -> Box<dyn Iterator<Item = (&K, &T)> + '_>;
}