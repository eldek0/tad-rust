use std::fmt::{Debug};

use crate::linked_list::node::Node;
use crate::linked_list::traits::linked_list_traits::LinkedListTrait;

pub struct LinkedList<T:Clone>{
    first:Option<Box<Node<T>>>,
    size:usize
}

impl <T: Clone> LinkedListTrait<T> for LinkedList<T> {
    fn new()-> LinkedList<T> {
        LinkedList{first:None, size:0}
    }

    fn new_from(values: Vec<T>)->LinkedList<T> {
        let mut list = LinkedList::new();
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

    fn get_mut(&mut self, index: usize) -> Result<&mut T, String> {
        if index >= self.size(){
            return Err(String::from("Index out of bounds error"));
        }
        Ok(&mut self.get_node_mut(index).value)
    }
    
    fn size(&self) -> usize {
        return self.size;
    }
}

impl <T: Clone> LinkedList<T>{
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

impl <T:Debug + Clone> Debug for LinkedList<T>{
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