use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use crate::heap::heap_error::HeapError;
use crate::heap::node::Node;
use crate::heap::traits::heap_traits::HeapTrait;

/// A generic heap data structure with configurable min or max priority ordering.
///
/// Each element is stored as a `(priority, value)` pair. The heap can operate
/// as a **min-heap** or a **max-heap** depending on the flag passed to [`Heap::new`].
/// It also resizes automatically when capacity is exceeded.
///
/// # Examples
///
/// ## Creating a min-heap and a max-heap
/// ```
/// use eldek_tad::heap::{heap::Heap, traits::heap_traits::HeapTrait};
/// 
/// let min_heap: Heap<i8, String> = Heap::new(true, 10);  // min-heap
/// let max_heap: Heap<i8, String> = Heap::new(false, 10); // max-heap
/// ```
///
/// ## Pushing elements — min-heap keeps lowest priority on top
/// ```
/// use eldek_tad::heap::{heap::Heap, traits::heap_traits::HeapTrait};
/// 
/// let mut heap: Heap<i8, &str> = Heap::new(true, 10);
/// heap.push(6, "Pasta");
/// heap.push(10, "Pizza");
///
/// assert_eq!(heap.peek().unwrap(), (&6, &"Pasta")); // lowest priority on top
/// assert_eq!(2, heap.size());
/// ```
///
/// ## Pushing elements — max-heap keeps highest priority on top
/// ```
/// use eldek_tad::heap::{heap::Heap, traits::heap_traits::HeapTrait};
/// 
/// let mut heap: Heap<i8, &str> = Heap::new(false, 10);
/// heap.push(6, "Pasta");
/// heap.push(10, "Pizza");
///
/// assert_eq!(heap.peek().unwrap(), (&10, &"Pizza")); // highest priority on top
/// ```
///
/// ## Popping elements (removes and returns the top)
/// ```
/// use eldek_tad::heap::{heap::Heap, traits::heap_traits::HeapTrait};
/// 
/// let mut heap: Heap<i8, &str> = Heap::new(true, 10); // min-heap
/// heap.push(10, "Hamburger");
/// heap.push(6, "Pasta");
///
/// assert_eq!(heap.pop().unwrap(), (6, "Pasta")); // lowest priority popped first
/// assert_eq!(1, heap.size());
/// ```
///
/// ## Popping in order from a max-heap
/// ```
/// use eldek_tad::heap::{heap::Heap, traits::heap_traits::HeapTrait};
/// 
/// let mut heap: Heap<i32, &str> = Heap::new(false, 10); // max-heap
/// heap.push(5, "A");
/// heap.push(10, "B");
/// heap.push(3, "C");
/// heap.push(8, "D");
///
/// assert_eq!(heap.pop().unwrap().0, 10);
/// assert_eq!(heap.pop().unwrap().0, 8);
/// assert_eq!(heap.pop().unwrap().0, 5);
/// assert_eq!(heap.pop().unwrap().0, 3);
/// ```
///
/// ## Peeking without consuming
/// ```
/// use eldek_tad::heap::{heap::Heap, traits::heap_traits::HeapTrait};
/// 
/// let mut heap: Heap<i8, &str> = Heap::new(true, 10);
/// heap.push(6, "Pasta");
/// heap.push(10, "Pizza");
///
/// assert_eq!(heap.peek().unwrap(), (&6, &"Pasta")); // does not remove the element
/// assert_eq!(2, heap.size());
/// ```
///
/// ## Handling errors on empty heap
/// ```
/// use eldek_tad::heap::{heap::Heap, traits::heap_traits::HeapTrait};
/// 
/// let mut heap: Heap<i8, &str> = Heap::new(true, 10);
///
/// assert!(heap.pop().is_err());
/// assert!(heap.peek().is_err());
/// ```
///
/// ## Automatic resizing
/// ```
/// use eldek_tad::heap::{heap::Heap, traits::heap_traits::HeapTrait};
/// 
/// let mut heap: Heap<i8, &str> = Heap::new(false, 2); // initial capacity of 2
/// heap.push(10, "Pizza");
/// heap.push(9, "Sushi");
/// heap.push(8, "Tacos"); // exceeds initial capacity, resizes automatically
///
/// assert_eq!(3, heap.size());
/// ```
///
/// ## Elements with the same priority are all stored
/// ```
/// use eldek_tad::heap::{heap::Heap, traits::heap_traits::HeapTrait};
/// 
/// let mut heap: Heap<i8, &str> = Heap::new(false, 10);
/// heap.push(10, "A");
/// heap.push(10, "B");
/// heap.push(10, "C");
///
/// assert_eq!(3, heap.size());
/// ```
///
/// ## Reuse after emptying
/// ```
/// use eldek_tad::heap::{heap::Heap, traits::heap_traits::HeapTrait};
/// 
/// let mut heap: Heap<i8, &str> = Heap::new(false, 10);
/// heap.push(5, "A");
/// heap.pop().unwrap();
///
/// heap.push(8, "B");
/// assert_eq!(heap.peek().unwrap().0, &8);
/// ```
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

    fn pop(&mut self) -> Result<(K, T), HeapError> {
        if self.size == 0 {
            return Err(HeapError::EmptyHeap);
        }

        let root:Node<K,T> = self.heap.pop_front().unwrap();

        self.size -= 1;
        if self.size > 0{
            self.heapify(0);
        }

        Ok((root.key, root.value))
    }

    fn peek(&self) -> Result<(&K, &T), HeapError> {
        if self.size == 0 {
            return Err(HeapError::EmptyHeap);
        }

        let node = self.get_node();
        Ok((&node.key, &node.value))
    }

    fn peek_mut(&mut self) -> Result<(&K, &mut T), HeapError> {
        if self.size == 0 {
            return Err(HeapError::EmptyHeap);
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
            panic!("{:?}", HeapError::EmptyHeap);
        }
        &self.heap[0]
    }
    fn get_node_mut(&mut self) -> &mut Node<K,T>{
        if self.size == 0{
            panic!("{:?}", HeapError::EmptyHeap);
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