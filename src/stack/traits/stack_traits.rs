use crate::stack::stack::Stack;

pub trait StackTrait<T:Clone>{
    fn new()->Stack<T>;
    fn push(&mut self, value:T);
    fn pop(&mut self)->Result<T, String>;
    fn peek(&self)->Result<T, String>;
    fn size(&self)->usize;
}