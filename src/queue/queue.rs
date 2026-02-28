use std::fmt::{Debug, Formatter};
use crate::linked_list::iter::IntoIter;
use crate::linked_list::linked_list::LinkedList;
use crate::linked_list::traits::linked_list_traits::LinkedListTrait;
use crate::queue::traits::queue_traits::QueueTrait;

pub struct Queue<T:Clone> {
    elements: LinkedList<T>,
    size: usize,
}

impl<T:Clone+PartialEq> QueueTrait<T> for Queue<T> {
    fn new() -> Self {
        Queue { elements: LinkedList::new(), size: 0 }
    }

    fn from_vec(values: Vec<T>)->Queue<T> {
        let mut queue: Queue<T> = Queue::new();
        for value in values{
            queue.enqueue(value);
        }
        queue
    }

    fn enqueue(&mut self, value: T) {
        self.elements.push(value);
    }

    fn dequeue(&mut self) -> Result<T, String> {
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
    
    fn is_empty(&self)->bool {
        return self.size() == 0;
    }
}

impl<T: Clone> IntoIterator for Queue<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<T:Debug + Clone> Debug for Queue<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.elements)
    }
}