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
            find_mut(parent.clone().unwrap());

        if parent_node.is_none() {
            return Err(format!("Parent key {:?} does not exist", parent));
        }

        if let Some(parent_node) = parent_node {
            if parent_node.left.is_none(){
                parent_node.left = Some(Box::new(node));
                return Ok(());
            }
            if parent_node.right.is_none(){
                parent_node.right = Some(Box::new(node));
                return Ok(());
            }

        }
        return Err(format!("Parent key {:?} does not have childs", parent));

    }

    fn delete(&mut self, key: &K) -> Result<(), String> {
        if self.first.is_none(){
            return Err(format!("Key {:?} does not exist", key));
        }

        if self.first.as_mut().unwrap().key.eq(key){
            self.first = None;
            return Ok(());
        }

        if let Some(parent) = self.first.as_mut().unwrap().find_parent(key.clone()){
            if let Some(left) = parent.left.as_ref(){
                if left.key.eq(key){
                    parent.left = None;
                    return Ok(());
                }
            }

            if let Some(right) = parent.right.as_ref(){
                if right.key.eq(key){
                    parent.right = None;
                    return Ok(());
                }
            }
        }

        return Err(format!("Key {:?} does not exist", key));
    }

    fn find(&self, key: &K) -> Result<(&K, &T), String> {
        if self.first.is_none(){
            return Err(format!("Key {:?} does not exist", key));
        }

        let node = self.first.as_ref().unwrap().find(key.clone());
        if node.is_none(){
            return Err(format!("Key {:?} does not exist", key));
        }
        return Ok((&node.unwrap().key, &node.unwrap().value));
    }

    fn find_mut(&mut self, key: &K) -> Result<(&K, &mut T), String> {
        if self.first.is_none(){
            return Err(format!("Key {:?} does not exist", key));
        }

        if let Some(node) = self.first.as_mut().unwrap().find_mut(key.clone()){
            return Ok((&node.key, &mut node.value));
        }
        else{
            return Err(format!("Key {:?} does not exist", key));
        }
        
    }

    fn size(&self) -> usize {
        Self::size(&self.first)
    }

    fn count_leaves(&self) -> usize {
        Self::count_leaves(&self.first)
    }
}

impl <K:PartialEq, T> BinaryTree<K, T>{
    fn size(root:&Option<Box<Node<K, T>>>) -> usize {
        if root.is_none(){
            return 0;
        }
        if let Some(root) = root {
            return 1 + Self::size(&root.right) + Self::size(&root.left);
        }
        return 0;
    }

    fn count_leaves(root:&Option<Box<Node<K, T>>>) -> usize {
        if root.is_none(){
            return 0;
        }
        if let Some(root) = root {
            if root.left.is_none() && root.right.is_none(){
                return 1;
            }
            return Self::count_leaves(&root.left) + Self::count_leaves(&root.right);
        }
        return 0;
    }
}

impl <K:Debug + PartialEq, T:Debug> Debug for BinaryTree<K,T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(first) = &self.first {
            writeln!(f, "{{")?;
            write!(f, "{:?}", first)?;
            write!(f, "}}")?;
        } else {
            write!(f, "{{}}")?;
        }

        Ok(())
    }
    
}