use crate::stack::{stack::Stack, stack_error::StackError};

pub trait StackTrait<T:Clone>{
    fn new()->Stack<T>;
    fn from_vec(values: Vec<T>)->Stack<T>;
    fn push(&mut self, value:T);
    fn pop(&mut self)->Result<T, StackError>;
    fn peek(&self)->Result<&T, StackError>;
    fn peek_mut(&mut self)->Result<&mut T, StackError>;
    fn size(&self)->usize;
    fn is_empty(&self)->bool;
}