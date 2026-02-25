pub trait HashMapTrait<K, T> {
    fn put(&mut self, key:K, value:T)->Result<(), String>;
    fn get(&self, key:K)->Result<(&K, &T), String>;
    fn get_mut(&mut self, key:K)->Result<(&K, &mut T), String>;
    fn contains(&self, key:K)->bool;
    fn remove(&mut self, key:K)->Result<(), String>;
}