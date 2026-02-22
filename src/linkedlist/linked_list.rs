use std::fmt::Display;

use crate::linkedlist::node::Node;
use crate::linkedlist::traits::linked_list_traits::LinkedlistTrait;

pub struct Linkedlist<T>{
    first:Option<Box<Node<T>>>,
    size:usize
}

impl <T> LinkedlistTrait<T> for Linkedlist<T> {
    fn new()-> Linkedlist<T> {
        Linkedlist{first:None, size:0}
    }

    fn add(&mut self, value: T){
        if self.first.is_none(){
            self.first = Some(Box::new(Node::new(value)));
        }
        else{
            // Adds a value to next of first
            let mut temp = self.first.as_mut().unwrap(); // it cant be none
            while temp.next.is_some() {
                temp = temp.next.as_mut().unwrap();
            }
            temp.next = Some(Box::new(Node::new(value)));
        }
        
        self.size += 1;
    }
    
    fn remove(&mut self, index: usize) {
        todo!()
    }
    
    fn get(&self, index: usize) -> Result<&T, String> {
        if index >= self.size{
            return Err(String::from("Index out of bounds error"));
        }

        let mut temp = self.first.as_ref().unwrap();
        for _ in 0..index {
            temp = temp.next.as_ref().unwrap();
        }

        Ok(&temp.value)
    }
    
    fn size(&self) -> usize {
        return self.size;
    }
}

impl <T:Display> Display for Linkedlist<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut temp = self.first.as_ref();
        write!(f, "[")?;
        while let Some(node) = temp{
            if node.next.is_none(){
                write!(f, "{}", node.value)?;
                break;
            }

            write!(f, "{}, ", node.value)?;
            temp = node.next.as_ref();
        }
        write!(f, "]")?;
        Ok(())
    }
}