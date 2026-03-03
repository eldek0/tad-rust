use crate::queue::{queue::Queue, queue_error::QueueError};


pub trait QueueTrait<T:Clone>{
    fn new() -> Queue<T>;
    fn from_vec(values: Vec<T>)->Queue<T>;
    fn enqueue(&mut self, value: T);
    fn dequeue(&mut self) -> Result<T, QueueError>;
    fn peek(&self) -> Result<&T, QueueError>;
    fn peek_mut(&mut self) -> Result<&mut T, QueueError>;
    fn size(&self) -> usize;
    fn is_empty(&self)->bool;
}