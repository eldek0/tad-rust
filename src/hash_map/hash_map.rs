use std::{fmt::{Debug, Formatter}, hash::{DefaultHasher, Hash, Hasher}};

use crate::{hash_map::{hash_map_error::HashMapError, node::Node, traits::hash_map_traits::HashMapTrait}, linked_list::{linked_list::LinkedList, traits::linked_list_traits::LinkedListTrait}};

/// A generic hash map data structure with configurable initial capacity.
///
/// Stores key-value pairs with O(1) average-case insertion, lookup, and deletion.
/// Handles hash collisions internally. Inserting an existing key overwrites its value.
///
/// # Examples
///
/// ## Creating a hash map
/// ```
/// use eldek_tad::hash_map::{hash_map::HashMap, traits::hash_map_traits::HashMapTrait};
/// 
/// let map: HashMap<i16, i16> = HashMap::new(10);
/// assert_eq!(0, map.size());
/// assert_eq!(10, map.capacity());
/// ```
///
/// ## Inserting key-value pairs
/// ```
/// use eldek_tad::hash_map::{hash_map::HashMap, traits::hash_map_traits::HashMapTrait};
/// 
/// let mut map: HashMap<i16, i16> = HashMap::new(10);
/// map.insert(1, 10);
/// map.insert(2, 12);
///
/// assert_eq!(2, map.size());
/// ```
///
/// ## Inserting an existing key overwrites the value
/// ```
/// use eldek_tad::hash_map::{hash_map::HashMap, traits::hash_map_traits::HashMapTrait};
/// 
/// let mut map: HashMap<i16, i16> = HashMap::new(10);
/// map.insert(1, 10);
/// map.insert(1, 20); // overwrites previous value
///
/// assert_eq!(1, map.size());
/// assert_eq!(&20, map.get(&1).unwrap());
/// ```
///
/// ## Getting a value by key
/// ```
/// use eldek_tad::hash_map::{hash_map::HashMap, traits::hash_map_traits::HashMapTrait};
/// 
/// let mut map: HashMap<i16, i16> = HashMap::new(10);
/// map.insert(1, 15);
///
/// assert_eq!(&15, map.get(&1).unwrap());
/// assert!(map.get(&99).is_err()); // non-existent key returns Err
/// ```
///
/// ## Getting a mutable reference
/// ```
/// use eldek_tad::hash_map::{hash_map::HashMap, traits::hash_map_traits::HashMapTrait};
/// 
/// let mut map: HashMap<i16, i16> = HashMap::new(10);
/// map.insert(1, 15);
///
/// if let Ok(val) = map.get_mut(&1) {
///     *val = 99;
/// }
/// assert_eq!(&99, map.get(&1).unwrap());
/// ```
///
/// ## Removing a key
/// ```
/// use eldek_tad::hash_map::{hash_map::HashMap, traits::hash_map_traits::HashMapTrait};
/// 
/// let mut map: HashMap<i16, i16> = HashMap::new(10);
/// map.insert(1, 15);
/// map.insert(2, 23);
/// map.remove(&1).unwrap();
///
/// assert_eq!(1, map.size());
/// assert!(map.remove(&50).is_err()); // removing non-existent key returns Err
/// ```
///
/// ## Checking if a key exists
/// ```
/// use eldek_tad::hash_map::{hash_map::HashMap, traits::hash_map_traits::HashMapTrait};
/// 
/// let mut map: HashMap<i16, i16> = HashMap::new(10);
/// map.insert(1, 10);
///
/// assert!(map.contains_key(&1));
/// assert!(!map.contains_key(&2));
/// ```
///
/// ## Collision handling
/// ```
/// use eldek_tad::hash_map::{hash_map::HashMap, traits::hash_map_traits::HashMapTrait};
/// 
/// let mut map: HashMap<i16, i16> = HashMap::new(5);
/// map.insert(1, 10); // 1 % 5 == 1
/// map.insert(6, 20); // 6 % 5 == 1, same bucket — handled internally
///
/// assert_eq!(2, map.size());
/// assert_eq!(&10, map.get(&1).unwrap());
/// assert_eq!(&20, map.get(&6).unwrap());
/// ```
///
/// ## Iterating over key-value pairs
/// ```
/// use eldek_tad::hash_map::{hash_map::HashMap, traits::hash_map_traits::HashMapTrait};
/// 
/// let mut map: HashMap<i32, i32> = HashMap::new(10);
/// map.insert(1, 10);
/// map.insert(2, 11);
/// map.insert(3, 12);
///
/// let mut sum = 0;
/// for (k, v) in map.iter() {
///     sum += *v;
/// }
/// assert_eq!(33, sum);
/// ```
pub struct HashMap<K, T>{
    hash: Vec<LinkedList<Node<K, T>>>, // For Separate Chaining
    size: usize,
    capacity: usize,
    load_factor:f32
}

impl <K: Hash + Debug + PartialEq + Clone, T> HashMapTrait<K, T> for HashMap<K, T>{
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

    fn get(&self, key: &K) -> Result<&T, HashMapError<K>>
    {
        let pos = self.hash_function(key);

        let bucket = self.hash
            .get(pos)
            .ok_or_else(|| HashMapError::KeyError { key: key.clone() })?;

        let node = bucket
            .iter()
            .find(|node| &node.key == key)
            .ok_or_else(|| HashMapError::KeyError { key: key.clone() })?;

        Ok(&node.value)
    }

    fn get_mut(&mut self, key: &K) -> Result<&mut T, HashMapError<K>>
    {
        let pos = self.hash_function(key);

        let bucket = self.hash
            .get_mut(pos)
            .ok_or_else(|| HashMapError::KeyError { key: key.clone() })?;

        let node = bucket
            .iter_mut()
            .find(|node| &node.key == key)
            .ok_or_else(|| HashMapError::KeyError { key: key.clone() })?;

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

    fn remove(&mut self, key: &K) -> Result<(), HashMapError<K>> {
        let pos = self.hash_function(key);
        let bucket = &mut self.hash[pos];

        if let Some(index) = bucket.iter().position(|node| &node.key == key) {
            bucket.remove(index).ok();
            self.size -= 1;
            return Ok(());
        }

        Err(HashMapError::KeyError { key:key.clone() })
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
        for (_, value) in self.hash.iter().enumerate() {
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