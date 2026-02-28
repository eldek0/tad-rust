use std::{fmt::{Debug, Formatter}, hash::Hash};

use crate::{hash_map::traits::hash_map_traits::HashMapTrait, linked_list::{linked_list::LinkedList, traits::linked_list_traits::LinkedListTrait}, prefix_tree::{node::Node, traits::prefix_tree_trait::PrefixTreeTrait}};

pub struct PrefixTree<T>{
    root: Node<T>,
    size: usize
}

impl <T:Hash + Debug + PartialEq + Clone> PrefixTreeTrait<T> for PrefixTree<T>{
    fn new()->Self {
        return PrefixTree { root: Node::new(), size: 0 }
    }

    fn insert(&mut self, sequence: impl IntoIterator<Item = T>) {
        let mut node = &mut self.root;
        for element in sequence{
            if !node.children.contains_key(&element){
                node.children.insert(element.clone(), Node::new());
            }
            node = node.children.get_mut(&element).unwrap();
        }
        node.is_end = true;
        self.size += 1;
    }

    fn insert_string(&mut self, sequence: &str) where T: From<char> {
        self.insert(PrefixTree::from_string_to_chars(sequence));
    }

    fn search(&self, sequence: impl IntoIterator<Item = T>)->bool {
        let mut node = &self.root;
        for element in sequence{
            if !node.children.contains_key(&element){
                return false;
            }
            node = node.children.get(&element).unwrap();
        }
        node.is_end
    }

    fn search_string(&self, sequence: &str)->bool where T:From<char> {
        return self.search(PrefixTree::from_string_to_chars(sequence));
    }

    fn remove(&mut self, sequence: impl IntoIterator<Item = T>) {
        let seq: Vec<T> = sequence.into_iter().collect();
        let removed = remove_recursive(&mut self.root, &seq, 0);
        if removed{
            self.size -= 1;
        }
    }
    
    fn remove_string(&mut self, sequence: &str) where T:From<char> {
        self.remove(PrefixTree::from_string_to_chars(sequence));
    }

    fn autocomplete(&self, prefix: impl IntoIterator<Item = T>) -> LinkedList<LinkedList<T>> {
        let mut node = &self.root;
        
        let mut prefixes: LinkedList<T> = LinkedList::new();
        for element in prefix {
            if !node.children.contains_key(&element) {
                return LinkedList::new();
            }
            node = node.children.get(&element).unwrap();
            prefixes.push(element);
        }
        
        return self.collect(node, &mut prefixes);
    }
    
    fn autocomplete_string(&self, prefix: &str) -> LinkedList<String>
    where 
        T: From<char>+Into<char> {
        let mut autocomplete = LinkedList::new();
        for word in self.autocomplete(PrefixTree::from_string_to_chars(prefix)) {
        let mut s = String::new();
        for j in 0..word.size() {
            let c: char = word.get(j).unwrap().clone().into();
            s.push(c);
        }
        autocomplete.push(s);
    }
    return autocomplete;
    }
    
    fn size(&self)->usize {
        self.size
    }

    fn is_empty(&self)->bool {
        self.size == 0
    }
}

impl<T: Hash + Debug + PartialEq + Clone> PrefixTree<T> {
    fn collect(&self, node: &Node<T>, current: &mut LinkedList<T>) -> LinkedList<LinkedList<T>> {
        let mut results: LinkedList<LinkedList<T>> = LinkedList::new();
        
        if node.is_end {
            results.push(current.clone());
        }
        
        for (element, child) in node.children.iter() {
            current.push(element.clone());
            let mut sub_results = self.collect(child, current);
            while sub_results.size() > 0 {
                if let Ok(item) = sub_results.remove(sub_results.size() - 1) {
                    results.push(item);
                }
            }
            current.remove(current.size() - 1).ok();
        }
        
        results
    }

    fn from_string_to_chars(sequence: &str) -> impl IntoIterator<Item = T> where T: From<char>{
        return sequence.chars().map(|c| T::from(c));
    }
}

fn remove_recursive<T: Hash + PartialEq + Clone + Debug>(node: &mut Node<T>, seq: &[T], index: usize) -> bool {
    if index == seq.len() {
        if !node.is_end {
            return false;
        }
        node.is_end = false;
        return node.children.is_empty();
    }

    let element = &seq[index];
    if !node.children.contains_key(element) {
        return false;
    }

    let child = node.children.get_mut(element).unwrap();
    let should_delete = remove_recursive(child, seq, index + 1);

    if should_delete {
        node.children.remove(element).ok();
        return !node.is_end && node.children.is_empty();
    }

    false
}

impl <T:Debug+Hash+PartialEq> Debug for PrefixTree<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.root.children)?;
        // for (val, node) in self.root.children.iter(){
        //     if node.is_end{
        //         write!(f, "{:?} end", val)?;
        //         break;
        //     }
        //     write!(f, "{:?}", node)?;
        // }
        Ok(())
    }
    
}