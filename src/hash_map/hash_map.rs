use std::{fmt::{Debug, Formatter}, hash::{DefaultHasher, Hash, Hasher}};

use crate::{hash_map::{node::Node, traits::hash_map_traits::HashMapTrait}, linked_list::{linked_list::LinkedList, traits::linked_list_traits::LinkedListTrait}};

pub struct HashMap<K, T>{
    hash: Vec<LinkedList<Node<K, T>>>, // For Separate Chaining
    size: usize,
    capacity: usize,
    load_factor:f32
}

impl <K: Hash + Debug + PartialEq, T> HashMapTrait<K, T> for HashMap<K, T>{
    fn new(capacity:usize)->Self {
        let mut hash = Vec::with_capacity(capacity);
        hash.resize_with(capacity, || LinkedList::new());
        HashMap {
            hash,
            size: 0,
            capacity,
            load_factor: 0.79 // 79%
        }
    }

    fn insert(&mut self, key:K, value:T){
        let pos = self.hash_function(&key);
        let bucket = &mut self.hash[pos];

        for node in bucket.iter_mut() {
            if node.key.eq(&key) {
                node.value = value;
                return;
            }
        }

        bucket.push(Node::new(key, value));
        self.size += 1;

        if self.size >= ((self.capacity as f32) * self.load_factor) as usize {
            self.resize();
        }
    }

    fn get(&self, key:&K)->Result<&T, String> {
        let pos = self.hash_function(key);

        let bucket = self.hash
            .get(pos)
            .ok_or_else(|| format!("Value with key {:?} does not exist", key))?;

        let node = bucket
            .iter()
            .find(|node| &node.key == key)
            .ok_or_else(|| format!("Value with key {:?} does not exist", key))?;

        Ok(&node.value)
    }

    fn get_mut(&mut self, key:&K)->Result<&mut T, String> {
        let pos = self.hash_function(key);

        let bucket = self.hash
            .get_mut(pos)
            .ok_or_else(|| format!("Value with key {:?} does not exist", key))?;

        let node = bucket
            .iter_mut()
            .find(|node| &node.key == key)
            .ok_or_else(|| format!("Value with key {:?} does not exist", key))?;

        Ok(&mut node.value)
    }

    fn contains_key(&self, key:&K)->bool {
        let pos = self.hash_function(key);

        if let Some(bucket) = self.hash.get(pos) {
            return bucket.iter().any(|node| &node.key == key);
        } else {
            return false;
        }
    }

    fn remove(&mut self, key: &K) -> Result<(), String> {
        let pos = self.hash_function(key);
        let bucket = &mut self.hash[pos];

        if let Some(index) = bucket.iter().position(|node| &node.key == key) {
            bucket.remove(index)?;
            self.size -= 1;
            return Ok(());
        }

        Err(format!("Value with key {:?} does not exist", key))
    }
    
    fn size(&self)->usize {
        return self.size;
    }
    
    fn capacity(&self)->usize {
        return self.capacity;
    }

    fn iter(&self) -> Box<dyn Iterator<Item = (&K, &T)> + '_> {
        Box::new(
            self.hash
                .iter()
                .flat_map(|bucket| {
                    bucket.iter().map(|node| (&node.key, &node.value))
                })
        )
    }
    
    fn is_empty(&self)->bool {
        return self.size() == 0;
    }
}

impl <K: Hash, T> HashMap<K, T>{
    fn resize(&mut self){
        let old = std::mem::take(&mut self.hash); // Takes and empties self.hash
        self.capacity *= 2;
        let mut new_hash = Vec::with_capacity(self.capacity);
        for temp in old{
            new_hash.push(temp);
        }

        self.hash = new_hash;
    }

    fn hash_code(&self, key: &K) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    fn hash_function(&self, key:&K)->usize{
        let hash_value = self.hash_code(&key);
        return (hash_value as usize) % self.capacity;
    }
}

/// Debug print
impl <K:Hash + Debug + PartialEq, T:Debug> Debug for HashMap<K, T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;

        let mut first = true;
        for (i, value) in self.hash.iter().enumerate() {
            if value.is_empty(){
                continue;
            }
            for v in value{
                if !first {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}: {:?}", v.key, v.value)?;
            }
            first = false;
        }

        write!(f, "}}")?;
        Ok(())
    
    }
}