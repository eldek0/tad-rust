use std::fmt::{Debug};

use crate::linked_list::iter::{IntoIter, Iter, IterMut};
use crate::linked_list::node::Node;
use crate::linked_list::traits::linked_list_traits::LinkedListTrait;

pub struct LinkedList<T>{
    first:Option<Box<Node<T>>>,
    size:usize
}

/// Public functions
impl <T:PartialEq> LinkedListTrait<T> for LinkedList<T> {
    fn new()-> LinkedList<T> {
        LinkedList{first:None, size:0}
    }

    fn from_vec(values: Vec<T>)->LinkedList<T> {
        let mut list = LinkedList::new();
        for value in values{
            list.push(value);
        }
        list
    }

    fn push(&mut self, value: T){
        self.insert(value, self.size()).ok();
    }

    fn insert(&mut self, value: T, index: usize)-> Result<(), String> {
        if index > self.size{
            return Err(format!("Index {:?} out of bounds", index));
        }

        let mut node = Node::new(value);
        if self.first.is_none(){
            self.first = Some(Box::new(node));
        }
        else if index == 0{
            node.next = self.first.take();
            self.first = Some(Box::new(node));
        }
        else{
            let mut temp = self.first.as_mut().unwrap(); // it cant be none
            for _ in 0..index - 1 {
                temp = temp.next.as_mut().unwrap();
            }
            let prev_next = temp.next.take();
            node.next = prev_next;
            temp.next = Some(Box::new(node));
        }
        
        self.size += 1;
        Ok(())
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

    fn contains(&self, value:&T)->bool{
        let mut next = self.first.as_deref();
        while let Some(node) = next{
            if &node.value == value{
                return true;
            }
            next = node.next.as_deref();
        }
        return false;
    }
    
    fn size(&self) -> usize {
        return self.size;
    }

    fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.first.as_deref(),
        }
    }

    fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            current: self.first.as_deref_mut(),
        }
    }
}

/// Private functions
impl <T> LinkedList<T>{
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

/// Iterators
impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {current:self.first}
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            current: self.first.as_deref(),
        }
    }
}

// mut
impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            current: self.first.as_deref_mut(),
        }
    }
}

impl<T:PartialEq> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = LinkedList::new();

        for item in iter {
            list.push(item);
        }

        list
    }
}

/// Debug print
impl <T:Debug> Debug for LinkedList<T>{
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