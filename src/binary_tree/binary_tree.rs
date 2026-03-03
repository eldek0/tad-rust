use std::fmt::{Debug, Formatter};
use crate::binary_tree::binary_tree_error::BinaryTreeError;
use crate::binary_tree::node::Node;
use crate::binary_tree::traits::binary_tree_traits::BinaryTreeTrait;
use crate::binary_tree::traits::node_traits::NodeTrait;

/// A generic binary tree storing key-value pairs with explicit parent assignment.
///
/// Nodes are placed manually by specifying a parent key
/// on insertion — there is no automatic ordering. Only one root (no parent) is allowed.
/// Deleting a node **cascades** and also removes its entire subtree.
///
/// # Examples
///
/// ## Creating a binary tree
/// ```
/// use eldek_tad::binary_tree::{binary_tree::BinaryTree, traits::binary_tree_traits::BinaryTreeTrait};
/// 
/// let tree: BinaryTree<i8, i8> = BinaryTree::new();
/// ```
///
/// ## Inserting nodes with explicit parent
/// ```
/// use eldek_tad::binary_tree::{binary_tree::BinaryTree, traits::binary_tree_traits::BinaryTreeTrait};
/// 
/// let mut tree: BinaryTree<i8, i8> = BinaryTree::new();
/// tree.insert(1, 10, None).unwrap();       // root, no parent
/// tree.insert(2, 20, Some(1)).unwrap();    // child of key 1
/// tree.insert(3, 30, Some(1)).unwrap();    // child of key 1
///
/// assert_eq!(3, tree.size());
/// ```
///
/// ## Only one root is allowed
/// ```
/// use eldek_tad::binary_tree::{binary_tree::BinaryTree, traits::binary_tree_traits::BinaryTreeTrait};
/// 
/// let mut tree: BinaryTree<i32, i32> = BinaryTree::new();
/// assert!(tree.insert(1, 10, None).is_ok());
/// assert!(tree.insert(2, 20, None).is_err()); // second root returns Err
/// assert_eq!(1, tree.size());
/// ```
///
/// ## Invalid parent key returns Err
/// ```
/// use eldek_tad::binary_tree::{binary_tree::BinaryTree, traits::binary_tree_traits::BinaryTreeTrait};
/// 
/// let mut tree: BinaryTree<i32, i32> = BinaryTree::new();
/// tree.insert(1, 10, None).unwrap();
///
/// assert!(tree.insert(2, 20, Some(999)).is_err()); // parent 999 does not exist
/// assert_eq!(1, tree.size());
/// ```
///
/// ## Duplicate keys are rejected
/// ```
/// use eldek_tad::binary_tree::{binary_tree::BinaryTree, traits::binary_tree_traits::BinaryTreeTrait};
/// 
/// let mut tree: BinaryTree<i32, i32> = BinaryTree::new();
/// tree.insert(1, 10, None).unwrap();
///
/// assert!(tree.insert(1, 999, Some(1)).is_err()); // duplicate key returns Err
/// assert_eq!(1, tree.size());
/// ```
///
/// ## Finding a node by key
/// ```
/// use eldek_tad::binary_tree::{binary_tree::BinaryTree, traits::binary_tree_traits::BinaryTreeTrait};
/// 
/// let mut tree: BinaryTree<i16, i16> = BinaryTree::new();
/// tree.insert(1, 10, None).unwrap();
/// tree.insert(2, 50, Some(1)).unwrap();
///
/// assert_eq!((&2, &50), tree.find(&2).unwrap());
/// assert!(tree.find(&999).is_err()); // non-existent key returns Err
/// ```
///
/// ## Finding a mutable reference to a value
/// ```
/// use eldek_tad::binary_tree::{binary_tree::BinaryTree, traits::binary_tree_traits::BinaryTreeTrait};
/// 
/// let mut tree: BinaryTree<i16, i16> = BinaryTree::new();
/// tree.insert(1, 10, None).unwrap();
/// tree.insert(2, 50, Some(1)).unwrap();
///
/// if let Ok((_, val)) = tree.find_mut(&2) {
///     *val = 99;
/// }
/// assert_eq!((&2, &99), tree.find(&2).unwrap());
/// ```
///
/// ## Deleting a node cascades its subtree
/// ```
/// use eldek_tad::binary_tree::{binary_tree::BinaryTree, traits::binary_tree_traits::BinaryTreeTrait};
/// 
/// let mut tree: BinaryTree<i16, i16> = BinaryTree::new();
/// tree.insert(1, 10, None).unwrap();
/// tree.insert(2, 50, Some(1)).unwrap();
/// tree.insert(3, 96, Some(2)).unwrap(); // child of 2
///
/// tree.delete(&2).unwrap(); // removes 2 and its child 3
/// assert_eq!(1, tree.size()); // only root (1) remains
///
/// assert!(tree.delete(&999).is_err()); // non-existent key returns Err
/// ```
///
/// ## Counting leaf nodes
/// ```
/// use eldek_tad::binary_tree::{binary_tree::BinaryTree, traits::binary_tree_traits::BinaryTreeTrait};
/// 
/// let mut tree: BinaryTree<i16, i16> = BinaryTree::new();
/// tree.insert(1, 10, None).unwrap();
/// tree.insert(2, 20, Some(1)).unwrap();
/// tree.insert(4, 40, Some(1)).unwrap();
/// tree.insert(3, 30, Some(2)).unwrap();
/// tree.insert(5, 50, Some(2)).unwrap();
/// tree.insert(6, 60, Some(4)).unwrap();
/// tree.insert(7, 70, Some(6)).unwrap();
///
/// assert_eq!(3, tree.count_leaves()); // nodes 3, 5, and 7 have no children
/// assert_eq!(0, BinaryTree::<i32, i32>::new().count_leaves());
/// ```
pub struct BinaryTree<K:PartialEq, T>{
    pub(super) first: Option<Box<Node<K, T>>>
}

impl <K:PartialEq + Clone + Debug, T> BinaryTreeTrait<K, T> for BinaryTree<K, T> {
    fn new() -> Self {
        BinaryTree { first: None }
    }

    fn insert(&mut self, key: K, value: T, parent:Option<K>)->Result<(), BinaryTreeError<K>> {
        let node:Node<K, T> = Node::new(key.clone(), value);

        if self.first.is_none(){
            self.first = Some(Box::new(node));
            return Ok(());
        }

        if let Some(first) = self.first.as_mut() {
            if first.find(key.clone()).is_some() {
                return Err(BinaryTreeError::KeyExistError { key });
            }
        }

        // If parent is none, then is the first tree element
        if parent.is_none() {
            if self.first.is_none() {
                self.first = Some(Box::new(node));
                return Ok(());
            } else {
                return Err(BinaryTreeError::RootError);
            }
        }


        let parent_node = self.first.
            as_mut().
            unwrap().
            find_mut(parent.clone().unwrap());

        if parent_node.is_none() {
            return Err(BinaryTreeError::ParentKeyError { parent_key: parent.unwrap() });
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
        return Err(BinaryTreeError::ParentWithNoChildsError { parent_key: parent.unwrap() });

    }

    fn delete(&mut self, key: &K) -> Result<(), BinaryTreeError<K>> {
        if self.first.is_none(){
            return Err(BinaryTreeError::KeyError { key: key.clone() });
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

        return Err(BinaryTreeError::KeyError { key: key.clone() });
    }

    fn find(&self, key: &K) -> Result<(&K, &T), BinaryTreeError<K>> {
        if self.first.is_none(){
            return Err(BinaryTreeError::KeyError { key: key.clone() });
        }

        let node = self.first.as_ref().unwrap().find(key.clone());
        if node.is_none(){
            return Err(BinaryTreeError::KeyError { key: key.clone() });
        }
        return Ok((&node.unwrap().key, &node.unwrap().value));
    }

    fn find_mut(&mut self, key: &K) -> Result<(&K, &mut T), BinaryTreeError<K>> {
        if self.first.is_none(){
            return Err(BinaryTreeError::KeyError { key: key.clone() });
        }

        if let Some(node) = self.first.as_mut().unwrap().find_mut(key.clone()){
            return Ok((&node.key, &mut node.value));
        }
        else{
            return Err(BinaryTreeError::KeyError { key: key.clone() });
        }
        
    }

    fn size(&self) -> usize {
        Self::size(&self.first)
    }

    fn count_leaves(&self) -> usize {
        Self::count_leaves(&self.first)
    }
    
    fn is_empty(&self)->bool {
        return self.size() == 0;
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