use crate::stack::stack::Stack;

pub trait StackTrait<T:Clone>{
    fn new()->Stack<T>;
    fn from_vec(values: Vec<T>)->Stack<T>;
    fn push(&mut self, value:T);
    fn pop(&mut self)->Result<T, String>;
    fn peek(&self)->Result<&T, String>;
    fn peek_mut(&mut self)->Result<&mut T, String>;
    fn size(&self)->usize;
    fn is_empty(&self)->bool;
}