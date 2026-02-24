use std::fmt::{Debug, Formatter};
use crate::binary_tree::node::Node;
use crate::binary_tree::traits::binary_tree_traits::BinaryTreeTrait;
use crate::binary_tree::traits::node_traits::NodeTrait;

pub struct BinaryTree<K:PartialEq, T>{
    first: Option<Box<Node<K, T>>>
}

impl <K:PartialEq + Clone + Debug, T> BinaryTreeTrait<K, T> for BinaryTree<K, T> {
    fn new() -> Self {
        BinaryTree { first: None }
    }

    fn insert(&mut self, key: K, value: T, parent:Option<K>)->Result<(),String> {
        let node:Node<K, T> = Node::new(key.clone(), value);

        if self.first.is_none(){
            self.first = Some(Box::new(node));
            return Ok(());
        }

        if let Some(first) = self.first.as_mut() {
            if first.find(key.clone()).is_some() {
                return Err(format!("Key {:?} already defined", key));
            }
        }

        // If parent is none, then is the first tree element
        if parent.is_none() {
            if self.first.is_none() {
                self.first = Some(Box::new(node));
                return Ok(());
            } else {
                return Err("Root already exists".to_string());
            }
        }


        let parent_node = self.first.
            as_mut().
            unwrap().
            find(parent.clone().unwrap());

        if parent_node.is_none() {
            return Err(format!("Parent key {:?} does not exist", parent));
        }

        if let Some(parent_node) = parent_node {
            if parent_node.left.is_none(){
                parent_node.left = Some(Box::new(node));
            }
            else if parent_node.right.is_none(){
                parent_node.right = Some(Box::new(node));
            }
            else {
                return Err(format!("Parent key {:?} does not have childs", parent));
            }

        }
        Ok(())

    }

    fn delete(&mut self, key: K) -> Result<(K, T), String> {
        todo!()
    }

    fn find(&self, key: K) -> Result<(&K, &T), String> {
        todo!()
    }

    fn size(&self) -> usize {
        Self::size_node(&self.first)
    }

    fn count_leaves(&self) -> usize {
        todo!()
    }
}

impl <K:PartialEq, T> BinaryTree<K, T>{
    fn size_node(root:&Option<Box<Node<K, T>>>) -> usize {
        if root.is_none(){
            return 0;
        }
        if let Some(root) = root {
            return 1 + Self::size_node(&root.right) + Self::size_node(&root.left);
        }
        return 0;
    }
}

impl <K:Debug + PartialEq, T:Debug> Debug for BinaryTree<K,T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"todo!")
    }
}