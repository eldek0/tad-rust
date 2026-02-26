pub trait HashMapTrait<K, T> {
    fn new(capacity:usize)->Self;
    fn put(&mut self, key:K, value:T)->Result<(), String>;
    fn get(&self, key:&K)->Result<(&K, &T), String>;
    fn get_mut(&mut self, key:&K)->Result<(&K, &mut T), String>;
    fn contains(&self, key:&K)->bool;
    fn remove(&mut self, key:&K)->Result<(), String>;
    fn size(&self)->usize;
    fn capacity(&self)->usize;
}