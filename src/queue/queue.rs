use std::fmt::{Debug, Formatter};
use crate::linked_list::iter::IntoIter;
use crate::linked_list::linked_list::LinkedList;
use crate::linked_list::traits::linked_list_traits::LinkedListTrait;
use crate::queue::queue_error::QueueError;
use crate::queue::traits::queue_traits::QueueTrait;

/// A generic queue data structure (FIFO - First In, First Out).
///
/// # Examples
///
/// ## Creating a queue
/// ```
/// use eldek_tad::queue::{queue::Queue, traits::queue_traits::QueueTrait};
/// 
/// let queue: Queue<i32> = Queue::new();
/// ```
///
/// ## Creating from a vector
/// ```
/// use eldek_tad::queue::{queue::Queue, traits::queue_traits::QueueTrait};
/// 
/// let queue: Queue<i16> = Queue::from_vec(vec![1, 2, 3]);
/// assert_eq!(3, queue.size());
/// ```
///
/// ## Enqueuing elements
/// ```
/// use eldek_tad::queue::{queue::Queue, traits::queue_traits::QueueTrait};
/// 
/// let mut queue = Queue::new();
/// queue.enqueue(1);
/// assert_eq!(1, queue.size());
/// ```
///
/// ## Dequeuing elements (front first)
/// ```
/// use eldek_tad::queue::{queue::Queue, traits::queue_traits::QueueTrait};
/// 
/// let mut queue = Queue::new();
/// queue.enqueue(10);
/// queue.enqueue(2);
///
/// assert_eq!(10, queue.dequeue().unwrap()); // returns the front element
/// assert_eq!(1, queue.size());
/// ```
///
/// ## Peeking without consuming
/// ```
/// use eldek_tad::queue::{queue::Queue, traits::queue_traits::QueueTrait};
/// 
/// let mut queue = Queue::new();
/// queue.enqueue(1);
///
/// assert_eq!(&1, queue.peek().unwrap()); // does not remove the element
/// assert_eq!(1, queue.size());
/// ```
///
/// ## Handling errors on empty queue
/// ```
/// use eldek_tad::queue::{queue::Queue, traits::queue_traits::QueueTrait};
/// 
/// let mut queue: Queue<i16> = Queue::new();
///
/// assert!(queue.dequeue().is_err());
/// assert!(queue.peek().is_err());
/// ```
///
/// ## Iterating (consumes the queue, front to back)
/// ```
/// use eldek_tad::queue::{queue::Queue, traits::queue_traits::QueueTrait};
/// 
/// let mut queue = Queue::new();
/// queue.enqueue(1);
/// queue.enqueue(2);
/// queue.enqueue(3);
///
/// let collected: Vec<i32> = queue.into_iter().collect();
/// assert_eq!(vec![1, 2, 3], collected); // front-first order
/// ```
///
/// ## Using iterator adapters
/// ```
/// use eldek_tad::queue::{queue::Queue, traits::queue_traits::QueueTrait};
/// 
/// let mut queue = Queue::new();
/// queue.enqueue(1);
/// queue.enqueue(2);
/// queue.enqueue(3);
///
/// let sum: i32 = queue.into_iter().sum();
/// assert_eq!(6, sum);
/// ```
pub struct Queue<T:Clone> {
    elements: LinkedList<T>
}

impl<T:Clone+PartialEq> QueueTrait<T> for Queue<T> {
    fn new() -> Self {
        Queue { elements: LinkedList::new() }
    }

    fn from_vec(values: Vec<T>)->Queue<T> {
        let mut queue: Queue<T> = Queue::new();
        for value in values{
            queue.enqueue(value);
        }
        queue
    }

    fn enqueue(&mut self, value: T) {
        self.elements.push(value);
    }

    fn dequeue(&mut self) -> Result<T, QueueError> {
        return self.elements.remove(0).map_err(|_| QueueError::EmptyQueue);
    }

    fn peek(&self) -> Result<&T, QueueError> {
        return self.elements.get(0).map_err(|_| QueueError::EmptyQueue);
    }

    fn peek_mut(&mut self) -> Result<&mut T, QueueError> {
        return self.elements.get_mut(0).map_err(|_| QueueError::EmptyQueue);
    }

    fn size(&self) -> usize {
        self.elements.size()
    }
    
    fn is_empty(&self)->bool {
        return self.size() == 0;
    }
}

impl<T: Clone> IntoIterator for Queue<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<T:Debug + Clone> Debug for Queue<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.elements)
    }
}