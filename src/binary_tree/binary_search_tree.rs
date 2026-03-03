use std::{fmt::{Debug, Formatter}, ops::{Deref, DerefMut}};

use crate::binary_tree::{binary_tree::BinaryTree, node::Node, traits::{binary_tree_traits::BinaryTreeTrait, node_traits::NodeTrait}};

/// A generic binary search tree (BST) storing key-value pairs.
///
/// Keys must be ordered (`Ord`). Duplicate keys are not allowed.
/// Deleting a node with children **cascades** and also removes its subtree.
///
/// # Examples
///
/// ## Creating a BST
/// ```
/// use eldek_tad::binary_tree::{binary_search_tree::BinarySearchTree, traits::binary_tree_traits::BinaryTreeTrait};
/// 
/// let tree: BinarySearchTree<i8, i8> = BinarySearchTree::new();
/// ```
///
/// ## Inserting key-value pairs
/// ```
/// use eldek_tad::binary_tree::{binary_search_tree::BinarySearchTree, traits::binary_tree_traits::BinaryTreeTrait};
/// 
/// let mut tree: BinarySearchTree<i8, i8> = BinarySearchTree::new();
/// tree.insert(4, 40).unwrap();
/// tree.insert(2, 20).unwrap();
/// tree.insert(6, 60).unwrap();
///
/// assert_eq!(3, tree.size());
/// ```
///
/// ## Keys are stored in sorted order
/// ```
/// use eldek_tad::binary_tree::{binary_search_tree::BinarySearchTree, traits::binary_tree_traits::BinaryTreeTrait};
/// 
/// let mut tree: BinarySearchTree<i16, i16> = BinarySearchTree::new();
/// tree.insert(4, 40).unwrap();
/// tree.insert(2, 20).unwrap();
/// tree.insert(6, 60).unwrap();
/// tree.insert(1, 10).unwrap();
/// tree.insert(3, 30).unwrap();
///
/// assert_eq!((&1, &10), tree.find(&1).unwrap());
/// assert_eq!((&3, &30), tree.find(&3).unwrap());
/// assert_eq!((&6, &60), tree.find(&6).unwrap());
/// ```
///
/// ## Duplicate keys are rejected
/// ```
/// use eldek_tad::binary_tree::{binary_search_tree::BinarySearchTree, traits::binary_tree_traits::BinaryTreeTrait};
/// 
/// let mut tree: BinarySearchTree<i32, i32> = BinarySearchTree::new();
/// assert!(tree.insert(1, 10).is_ok());
/// assert!(tree.insert(1, 999).is_err()); // duplicate key returns Err
/// assert_eq!(1, tree.size());
/// ```
///
/// ## Finding a key
/// ```
/// use eldek_tad::binary_tree::{binary_search_tree::BinarySearchTree, traits::binary_tree_traits::BinaryTreeTrait};
/// 
/// let mut tree: BinarySearchTree<i16, i16> = BinarySearchTree::new();
/// tree.insert(4, 40).unwrap();
/// tree.insert(2, 20).unwrap();
///
/// assert_eq!((&2, &20), tree.find(&2).unwrap());
/// assert!(tree.find(&999).is_err()); // non-existent key returns Err
/// ```
///
/// ## Finding a mutable reference to a value
/// ```
/// use eldek_tad::binary_tree::{binary_search_tree::BinarySearchTree, traits::binary_tree_traits::BinaryTreeTrait};
/// 
/// let mut tree: BinarySearchTree<i16, i16> = BinarySearchTree::new();
/// tree.insert(2, 20).unwrap();
///
/// if let Ok((_, val)) = tree.find_mut(&2) {
///     *val = 99;
/// }
/// assert_eq!((&2, &99), tree.find(&2).unwrap());
/// ```
///
/// ## Deleting a node (cascades subtree)
/// ```
/// use eldek_tad::binary_tree::{binary_search_tree::BinarySearchTree, traits::binary_tree_traits::BinaryTreeTrait};
/// 
/// let mut tree: BinarySearchTree<i16, i16> = BinarySearchTree::new();
/// tree.insert(4, 40).unwrap();
/// tree.insert(2, 20).unwrap();
/// tree.insert(1, 10).unwrap(); // child of 2
/// tree.insert(3, 30).unwrap(); // child of 2
///
/// tree.delete(&2).unwrap(); // deletes 2 and its entire subtree (1 and 3)
/// assert_eq!(1, tree.size()); // only root (4) remains
///
/// assert!(tree.delete(&999).is_err()); // non-existent key returns Err
/// ```
///
/// ## Counting leaf nodes
/// ```
/// use eldek_tad::binary_tree::{binary_search_tree::BinarySearchTree, traits::binary_tree_traits::BinaryTreeTrait};
/// 
/// let mut tree: BinarySearchTree<i16, i16> = BinarySearchTree::new();
/// tree.insert(4, 40).unwrap();
/// tree.insert(2, 20).unwrap();
/// tree.insert(6, 60).unwrap();
/// tree.insert(1, 10).unwrap();
/// tree.insert(3, 30).unwrap();
///
/// assert_eq!(3, tree.count_leaves()); // nodes 1, 3, and 6 have no children
/// assert_eq!(0, BinarySearchTree::<i32, i32>::new().count_leaves());
/// ```
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