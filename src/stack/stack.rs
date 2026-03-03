use std::{fmt::Debug};

use crate::{linked_list::{iter::IntoIter, linked_list::LinkedList, traits::linked_list_traits::LinkedListTrait}, stack::{stack_error::StackError, traits::stack_traits::StackTrait}};

/// A generic stack data structure (LIFO - Last In, First Out).
///
/// # Examples
///
/// ## Creating a stack
/// ```
/// use eldek_tad::stack::{stack::Stack, traits::stack_traits::StackTrait};
/// 
/// let stack: Stack<i32> = Stack::new();
/// ```
///
/// ## Creating from a vector
/// ```
/// use eldek_tad::stack::{stack::Stack, traits::stack_traits::StackTrait};
/// 
/// let stack: Stack<i16> = Stack::from_vec(vec![1, 2, 3]);
/// assert_eq!(3, stack.size());
/// ```
///
/// ## Pushing and popping elements
/// ```
/// use eldek_tad::stack::{stack::Stack, traits::stack_traits::StackTrait};
/// 
/// let mut stack = Stack::new();
/// stack.push(10);
/// stack.push(2);
///
/// assert_eq!(2, stack.pop().unwrap()); // returns the top element
/// assert_eq!(1, stack.size());
/// ```
///
/// ## Peeking without consuming
/// ```
/// use eldek_tad::stack::{stack::Stack, traits::stack_traits::StackTrait};
/// 
/// let mut stack = Stack::new();
/// stack.push(1);
///
/// assert_eq!(&1, stack.peek().unwrap()); // does not remove the element
/// assert_eq!(1, stack.size());
/// ```
///
/// ## Handling errors on empty stack
/// ```
/// use eldek_tad::stack::{stack::Stack, traits::stack_traits::StackTrait};
/// 
/// let mut stack: Stack<i16> = Stack::new();
///
/// assert!(stack.pop().is_err());
/// assert!(stack.peek().is_err());
/// ```
///
/// ## Iterating (consumes the stack, top to bottom)
/// ```
/// use eldek_tad::stack::{stack::Stack, traits::stack_traits::StackTrait};
/// 
/// let mut stack = Stack::new();
/// stack.push(1);
/// stack.push(2);
/// stack.push(3);
///
/// let collected: Vec<i32> = stack.into_iter().collect();
/// assert_eq!(vec![3, 2, 1], collected); // top-first order
/// ```
///
/// ## Using iterator adapters
/// ```
/// use eldek_tad::stack::{stack::Stack, traits::stack_traits::StackTrait};
/// 
/// let mut stack = Stack::new();
/// stack.push(1);
/// stack.push(2);
/// stack.push(3);
///
/// let sum: i32 = stack.into_iter().sum();
/// assert_eq!(6, sum);
/// ```
pub struct Stack<T:Clone> {
    elements:LinkedList<T>
}

impl <T:Clone+PartialEq> StackTrait<T> for Stack<T> {
    fn new()->Stack<T> {
        return Stack { elements: LinkedList::new() }
    }

    fn from_vec(values: Vec<T>)->Stack<T> {
        let mut stack = Stack::new();
        for value in values{
            stack.push(value);
        }
        stack
    }

    fn push(&mut self, value:T) {
        self.elements.insert(value, 0).ok();
    }

    fn pop(&mut self)->Result<T, StackError> {
        return self.elements.remove(0).map_err(|_| StackError::EmptyStack);
    }

    fn peek(&self)->Result<&T, StackError> {
        if self.is_empty(){return Err(StackError::EmptyStack);}
        let index = self.elements.size()-1;
        return self.elements.get(index).map_err(|_| StackError::EmptyStack);
    }

    fn peek_mut(&mut self)->Result<&mut T, StackError> {
        let index = self.elements.size()-1;
        return self.elements.get_mut(index).map_err(|_| StackError::EmptyStack);
    }

    fn size(&self)->usize {
        return self.elements.size();
    }
    
    fn is_empty(&self)->bool {
        return self.size() == 0;
    }
}

impl<T: Clone> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl <T:Debug + Clone> Debug for Stack<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.elements)?;
        Ok(())
    }
}