use std::{fmt::{Debug, Formatter}, ops::{Deref, DerefMut}};

use crate::binary_tree::{binary_tree::BinaryTree, node::Node, traits::{binary_tree_traits::BinaryTreeTrait, node_traits::NodeTrait}};

pub struct BinarySearchTree<K:PartialEq, T>(BinaryTree<K, T>);

impl<K:PartialEq, T> Deref for BinarySearchTree<K, T> {
    type Target = BinaryTree<K, T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K:PartialEq, T> DerefMut for BinarySearchTree<K, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl <K:PartialOrd+Clone+Debug, T> BinarySearchTree<K, T>{
    pub fn new() -> Self {
        BinarySearchTree(BinaryTree::new())
    }

    // Overrides the BinaryTree's insert function
    pub fn insert(&mut self, key: K, value: T)->Result<(),String> {
        let node:Node<K, T> = Node::new(key.clone(), value);

        if self.first.is_none(){
            self.first = Some(Box::new(node));
            return Ok(());
        }

        let mut current = self.first.as_mut().unwrap();

        loop {
            if key == current.key {
                return Err(format!("Key {:?} already defined", key));
            } else if key < current.key {
                if current.left.is_none() {
                    current.left = Some(Box::new(node));
                    return Ok(());
                }
                current = current.left.as_mut().unwrap();
            } else {
                if current.right.is_none() {
                    current.right = Some(Box::new(node));
                    return Ok(());
                }
                current = current.right.as_mut().unwrap();
            }
        }

    } 
}

impl <K:Debug + PartialEq, T:Debug> Debug for BinarySearchTree<K,T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0) // Delegates the write to BinaryTree (self.0)
    }
    
}