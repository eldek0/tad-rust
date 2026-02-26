use std::fmt::{Debug, Formatter};
use crate::linked_list::linked_list::LinkedList;
use crate::linked_list::traits::linked_list_traits::LinkedListTrait;
use crate::queue::traits::queue_traits::QueueTrait;

pub struct Queue<T:Clone> {
    elements: LinkedList<T>,
    size: usize,
}

impl<T:Clone> QueueTrait<T> for Queue<T> {
    fn new() -> Self {
        Queue { elements: LinkedList::new(), size: 0 }
    }

    fn push(&mut self, value: T) {
        self.elements.push(value);
    }

    fn pop(&mut self) -> Result<T, String> {
        if self.size() == 0{
            return Err(String::from("Empty queue exception"));
        }

        let first = self.elements.get(0)?.clone();
        let _ = self.elements.remove(0);
        Ok(first)
    }

    fn peek(&self) -> Result<&T, String> {
        if self.size() == 0{
            return Err(String::from("Empty queue exception"));
        }

        return self.elements.get(0);
    }

    fn peek_mut(&mut self) -> Result<&mut T, String> {
        if self.size() == 0{
            return Err(String::from("Empty queue exception"));
        }

        return self.elements.get_mut(0);
    }

    fn size(&self) -> usize {
        self.elements.size()
    }
}

impl<T:Debug + Clone> Debug for Queue<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.elements)
    }
}