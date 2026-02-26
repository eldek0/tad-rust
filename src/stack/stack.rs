use std::fmt::{Debug};

use crate::{linked_list::{linked_list::LinkedList, traits::linked_list_traits::LinkedListTrait}, stack::traits::stack_traits::StackTrait};

pub struct Stack<T:Clone> {
    elements:LinkedList<T>,
    size:usize,
}

impl <T:Clone> StackTrait<T> for Stack<T> {
    fn new()->Stack<T> {
        return Stack { elements: LinkedList::new(), size: 0 }
    }

    fn push(&mut self, value:T) {
        self.elements.push(value);
    }

    fn pop(&mut self)->Result<T, String> {
        if self.elements.size() == 0 {
            return Err(String::from("Empty stack exception"));
        }

        let index = self.elements.size()-1;
        let last = self.elements.get(index)?.clone();
        let _ = self.elements.remove(index);
        return Ok(last);
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

impl <T:Debug + Clone> Debug for Stack<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.elements)?;
        Ok(())
    }
}