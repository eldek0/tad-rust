use std::fmt::{Debug};

use crate::linked_list::iter::{IntoIter, Iter, IterMut};
use crate::linked_list::linked_list_error::LinkedListError;
use crate::linked_list::node::Node;
use crate::linked_list::traits::linked_list_traits::LinkedListTrait;

/// A generic singly linked list data structure.
///
/// Supports index-based access, insertion, removal, and multiple iteration modes.
///
/// # Examples
///
/// ## Creating a linked list
/// ```
/// use eldek_tad::linked_list::{linked_list::LinkedList, traits::linked_list_traits::LinkedListTrait};
/// 
/// let list: LinkedList<i32> = LinkedList::new();
/// ```
///
/// ## Creating from a vector
/// ```
/// use eldek_tad::linked_list::{linked_list::LinkedList, traits::linked_list_traits::LinkedListTrait};
/// 
/// let list: LinkedList<i32> = LinkedList::from_vec(vec![1, 2, 3, 4, 5]);
/// assert_eq!(5, list.size());
/// ```
///
/// ## Collecting from an iterator
/// ```
/// use eldek_tad::linked_list::{linked_list::LinkedList, traits::linked_list_traits::LinkedListTrait};
/// 
/// let list: LinkedList<i32> = vec![5, 6, 7].into_iter().collect();
/// assert_eq!(3, list.size());
/// ```
///
/// ## Pushing elements
/// ```
/// use eldek_tad::linked_list::{linked_list::LinkedList, traits::linked_list_traits::LinkedListTrait};
/// 
/// let mut list = LinkedList::new();
/// list.push(10);
/// list.push(20);
/// assert_eq!(2, list.size());
/// ```
///
/// ## Accessing elements by index
/// ```
/// use eldek_tad::linked_list::{linked_list::LinkedList, traits::linked_list_traits::LinkedListTrait};
/// 
/// let list: LinkedList<i16> = LinkedList::from_vec(vec![1, 2, 3]);
///
/// assert_eq!(&2, list.get(1).unwrap());
/// assert!(list.get(10).is_err()); // out of bounds returns Err
/// ```
///
/// ## Checking if an element exists
/// ```
/// use eldek_tad::linked_list::{linked_list::LinkedList, traits::linked_list_traits::LinkedListTrait};
/// 
/// let list: LinkedList<i16> = LinkedList::from_vec(vec![10, 20, 30]);
///
/// assert!(list.contains(&10));
/// assert!(!list.contains(&99));
/// ```
///
/// ## Inserting at a specific index
/// ```
/// use eldek_tad::linked_list::{linked_list::LinkedList, traits::linked_list_traits::LinkedListTrait};
/// 
/// let mut list: LinkedList<i32> = LinkedList::from_vec(vec![1, 3, 4]);
/// list.insert(2, 1).unwrap(); // insert value 2 at index 1
///
/// let collected: Vec<i32> = list.iter().cloned().collect();
/// assert_eq!(vec![1, 2, 3, 4], collected);
///
/// assert!(list.insert(99, 100).is_err()); // out of bounds returns Err
/// ```
///
/// ## Removing elements by index
/// ```
/// use eldek_tad::linked_list::{linked_list::LinkedList, traits::linked_list_traits::LinkedListTrait};
/// 
/// let mut list: LinkedList<i32> = LinkedList::from_vec(vec![10, 20, 30]);
///
/// list.remove(1).unwrap(); // removes element at index 1
/// assert_eq!(2, list.size());
/// assert_eq!(&30, list.get(1).unwrap());
///
/// assert!(list.remove(99).is_err()); // out of bounds returns Err
/// ```
///
/// ## Iterating (consumes the list)
/// ```
/// use eldek_tad::linked_list::{linked_list::LinkedList, traits::linked_list_traits::LinkedListTrait};
/// 
/// let mut list = LinkedList::new();
/// list.push(1);
/// list.push(2);
/// list.push(3);
///
/// let mut iter = list.into_iter();
/// assert_eq!(Some(1), iter.next());
/// assert_eq!(Some(2), iter.next());
/// assert_eq!(Some(3), iter.next());
/// assert_eq!(None, iter.next());
/// ```
///
/// ## Iterating by reference
/// ```
/// use eldek_tad::linked_list::{linked_list::LinkedList, traits::linked_list_traits::LinkedListTrait};
/// 
/// let mut list = LinkedList::new();
/// list.push(10);
/// list.push(20);
///
/// let sum: i32 = (&list).into_iter().sum(); // list is not consumed
/// assert_eq!(30, sum);
/// assert_eq!(2, list.size()); // still accessible
/// ```
///
/// ## Iterating mutably
/// ```
/// use eldek_tad::linked_list::{linked_list::LinkedList, traits::linked_list_traits::LinkedListTrait};
/// 
/// let mut list = LinkedList::new();
/// list.push(1);
/// list.push(2);
/// list.push(3);
///
/// for x in &mut list {
///     *x *= 2;
/// }
///
/// let collected: Vec<i32> = list.into_iter().collect();
/// assert_eq!(vec![2, 4, 6], collected);
/// ```
///
/// ## Using for loop syntax
/// ```
/// use eldek_tad::linked_list::{linked_list::LinkedList, traits::linked_list_traits::LinkedListTrait};
/// 
/// let list: LinkedList<i32> = vec![1, 2, 3].into_iter().collect();
///
/// let mut sum = 0;
/// for x in &list {
///     sum += *x;
/// }
/// assert_eq!(6, sum);
/// ```
#[derive(PartialEq, Clone)]
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

    fn insert(&mut self, value: T, index: usize)-> Result<(), LinkedListError> {
        if index > self.size{
            return Err(LinkedListError::OutOfBounds { index });
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
    
    fn remove(&mut self, index: usize) -> Result<T, LinkedListError>{
        if index >= self.size(){
            return Err(LinkedListError::OutOfBounds { index });
        }

        let removed_value;

        if index == 0 {
            let node = self.first.take().unwrap();
            removed_value = node.value;
            self.first = node.next;
        }
        else if index == self.size() - 1 {
            let last = self.get_node_mut(self.size() - 2);
            let node = last.next.take().unwrap();
            removed_value = node.value;
        }
        else {
            let node_before = self.get_node_mut(index - 1);
            let node_to_remove = node_before.next.take().unwrap();
            node_before.next = node_to_remove.next;
            removed_value = node_to_remove.value;
        }

        self.size -= 1;
        Ok(removed_value)
    }

    fn get(&self, index: usize) -> Result<&T, LinkedListError> {
        if index >= self.size(){
            return Err(LinkedListError::OutOfBounds { index });
        }
        Ok(&self.get_node(index).value)
    }

    fn get_mut(&mut self, index: usize) -> Result<&mut T, LinkedListError> {
        if index >= self.size(){
            return Err(LinkedListError::OutOfBounds { index });
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
    
    fn is_empty(&self)->bool {
        return self.size() == 0;
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