use std::{fmt::Debug};

use crate::{linked_list::{iter::IntoIter, linked_list::LinkedList, traits::linked_list_traits::LinkedListTrait}, stack::traits::stack_traits::StackTrait};

pub struct Stack<T:Clone> {
    elements:LinkedList<T>,
    size:usize,
}

impl <T:Clone> StackTrait<T> for Stack<T> {
    fn new()->Stack<T> {
        return Stack { elements: LinkedList::new(), size: 0 }
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

    fn pop(&mut self)->Result<T, String> {
        if self.elements.size() == 0 {
            return Err(String::from("Empty stack exception"));
        }

        let first = self.elements.get(0)?.clone();
        let _ = self.elements.remove(0);
        return Ok(first);
    }

    fn peek(&self)->Result<&T, String> {
        if self.elements.size() == 0 {
            return Err(String::from("Empty stack exception"));
        }
        
        let index = self.elements.size()-1;
        let last = self.elements.get(index);
        return last;
    }

    fn peek_mut(&mut self)->Result<&mut T, String> {
        if self.elements.size() == 0 {
            return Err(String::from("Empty stack exception"));
        }
        
        let index = self.elements.size()-1;
        return self.elements.get_mut(index);
    }

    fn size(&self)->usize {
        return self.elements.size();
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