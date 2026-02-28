use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use crate::heap::node::Node;
use crate::heap::traits::heap_traits::HeapTrait;

pub struct Heap<K: PartialOrd, T>{
    heap: VecDeque<Node<K, T>>,
    min: bool,
    size: usize,
}

impl<K: PartialOrd, T> HeapTrait<K, T> for Heap<K, T>{
    fn new(min:bool, capacity:usize) -> Self {
        Heap{
            heap: VecDeque::with_capacity(capacity),
            min,
            size: 0,
        }
    }

    fn push(&mut self, key: K, value: T) {
        if self.size >= self.heap.len() {
            self.resize();
        }

        let new_node:Node<K, T> = Node::new(key, value);
        self.heap.push_back(new_node);
        self.size += 1;

        // Order the heap
        let mut pos = self.size - 1;
        while pos > 0{
            let father_pos = (pos - 1) / 2;
            if self.compare_is_node_min(self.heap.get(pos), self.heap.get(father_pos)) {
                self.swap(pos, father_pos);
                pos = father_pos;
            } else { break; }
        }
    }

    fn pop(&mut self) -> Result<(K, T), String> {
        if self.size == 0 {
            return Err(String::from("Index out of bounds error"));
        }

        let root:Node<K,T> = self.heap.pop_front().unwrap();

        self.size -= 1;
        if self.size > 0{
            self.heapify(0);
        }

        Ok((root.key, root.value))
    }

    fn peek(&self) -> Result<(&K, &T), String> {
        if self.size == 0 {
            return Err(String::from("Index out of bounds error"));
        }

        let node = self.get_node();
        Ok((&node.key, &node.value))
    }

    fn peek_mut(&mut self) -> Result<(&K, &mut T), String> {
        if self.size == 0 {
            return Err(String::from("Index out of bounds error"));
        }

        let node = self.get_node_mut();
        Ok((&node.key, &mut node.value))
    }

    fn size(&self) -> usize {
        self.size
    }
    
    fn is_empty(&self)->bool {
        return self.size() == 0;
    }
    
}

impl<K: PartialOrd, T> Heap<K, T>{
    fn resize(&mut self){
        let new_capacity = self.heap.capacity()*2;
        let mut new_heap:VecDeque<Node<K, T>> = VecDeque::with_capacity(new_capacity);
        for node in self.heap.drain(0..){
            new_heap.push_back(node);
        }
        self.heap = new_heap;
    }
    fn heapify(&mut self, index:usize){
        let left = index*2 + 1;
        let right = index*2 + 2;
        let mut endpoint = index; // max / min



        if left < self.size &&
            self.compare_is_node_min(self.heap.get(left), self.heap.get(endpoint)){
            endpoint = left;
        }

        if right < self.size &&
            self.compare_is_node_min(self.heap.get(right), self.heap.get(endpoint)){
            endpoint = right;
        }

        if endpoint != index{
            self.swap(endpoint, index);
            self.heapify(endpoint);
        }
    }

    fn compare_is_node_min(&self, node1:Option<&Node<K, T>>, node2:Option<&Node<K, T>>) -> bool {
        // Is min heap
        if self.min{
            return node1 < node2;
        }
        // Is max heap
        node1 > node2
    }

    fn get_node(&self) -> &Node<K,T>{
        if self.size == 0{
            panic!("Index out of bounds error");
        }
        &self.heap[0]
    }
    fn get_node_mut(&mut self) -> &mut Node<K,T>{
        if self.size == 0{
            panic!("Index out of bounds error");
        }
        &mut self.heap[0]
    }
    fn swap(&mut self, i:usize, j:usize){
        self.heap.swap(i, j);
    }
}

impl <K: Debug + PartialOrd, T: Debug> Debug for Heap<K, T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        for (i, value) in self.heap.iter().enumerate() {
            if i == self.size() - 1 {
                write!(f, "{:?}", value)?;
            } else {
                write!(f, "{:?}, ", value)?;
            }
        }

        write!(f, "]")?;
        Ok(())
    }
}