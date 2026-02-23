use std::fmt::{Debug};

use crate::linkedlist::node::Node;
use crate::linkedlist::traits::linkedlist_traits::LinkedlistTrait;

pub struct Linkedlist<T:Clone>{
    first:Option<Box<Node<T>>>,
    size:usize
}

impl <T: Clone> LinkedlistTrait<T> for Linkedlist<T> {
    fn new()-> Linkedlist<T> {
        Linkedlist{first:None, size:0}
    }

    fn new_from(values: Vec<T>)->Linkedlist<T> {
        let mut list = Linkedlist::new();
        for value in values{
            list.add(value);
        }
        list
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
    
    fn remove(&mut self, index: usize) -> Result<(), String>{
        if index >= self.size(){
            return Err(String::from("Index out of bounds error"));
        }

        if index == 0 {
            self.first = self.first.take().unwrap().next;
        }
        else if index == self.size() - 1 {
            let last = self.get_node_mut(self.size() - 2);
            last.next = None;
        }
        else {
            let node_before = self.get_node_mut(index-1);
            let node_to_remove = node_before.next.take().unwrap();
            node_before.next = node_to_remove.next;
        }

        self.size -= 1;
        Ok(())
    }

    fn get(&self, index: usize) -> Result<&T, String> {
        if index >= self.size(){
            return Err(String::from("Index out of bounds error"));
        }
        Ok(&self.get_node(index).value)
    }
    
    fn size(&self) -> usize {
        return self.size;
    }
}

impl <T: Clone> Linkedlist<T>{
    fn get_node(&self, index: usize) -> &Node<T>{
        if index >= self.size{
            panic!("Index out of bounds error"); // for private fn is ok?
        }

        let mut temp = self.first.as_ref().unwrap();
        for _ in 0..index {
            temp = temp.next.as_ref().unwrap();
        }

        &temp
    }

    fn get_node_mut(&mut self, index: usize) -> &mut Node<T>{
        if index >= self.size{
            panic!("Index out of bounds error");
        }

        let mut temp = self.first.as_mut().unwrap();
        for _ in 0..index {
            temp = temp.next.as_mut().unwrap();
        }

        temp
    }
}

impl <T:Debug + Clone> Debug for Linkedlist<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut temp = self.first.as_ref();
        write!(f, "[")?;
        while let Some(node) = temp{
            if node.next.is_none(){
                write!(f, "{:?}", node.value)?;
                break;
            }

            write!(f, "{:?}, ", node.value)?;
            temp = node.next.as_ref();
        }
        write!(f, "]")?;
        Ok(())
    }
}