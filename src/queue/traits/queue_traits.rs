pub trait QueueTrait<T>{
    fn new() -> Self;
    fn push(&mut self, value: T);
    fn pop(&mut self) -> Result<T, String>;
    fn peek(&self) -> Result<T, String>;
    fn size(&self) -> usize;
}