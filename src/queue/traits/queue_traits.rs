use crate::queue::queue::Queue;


pub trait QueueTrait<T:Clone>{
    fn new() -> Queue<T>;
    fn from_vec(values: Vec<T>)->Queue<T>;
    fn enqueue(&mut self, value: T);
    fn dequeue(&mut self) -> Result<T, String>;
    fn peek(&self) -> Result<&T, String>;
    fn peek_mut(&mut self) -> Result<&mut T, String>;
    fn size(&self) -> usize;
    fn is_empty(&self)->bool;
}