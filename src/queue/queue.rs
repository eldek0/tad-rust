use std::fmt::{Display, Formatter};
use crate::linkedlist::linkedlist::Linkedlist;
use crate::linkedlist::traits::linkedlist_traits::LinkedlistTrait;
use crate::queue::traits::queue_traits::QueueTrait;

pub struct Queue<T:Clone> {
    elements: Linkedlist<T>,
    size: usize,
}

impl<T:Clone> QueueTrait<T> for Queue<T> {
    fn new() -> Self {
        Queue { elements: Linkedlist::new(), size: 0 }
    }

    fn push(&mut self, value: T) {
        self.elements.add(value);
    }

    fn pop(&mut self) -> Result<T, String> {
        if self.size() == 0{
            return Err(String::from("Empty queue exception"));
        }

        let first = self.elements.get(0)?.clone();
        let _ = self.elements.remove(0);
        Ok(first)
    }

    fn peek(&self) -> Result<T, String> {
        if self.size() == 0{
            return Err(String::from("Empty queue exception"));
        }

        let first = self.elements.get(0)?.clone();
        Ok(first)
    }

    fn size(&self) -> usize {
        self.elements.size()
    }
}

impl<T:Display + Clone> Display for Queue<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.elements)
    }
}