#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

use crate::{binary_tree::{binary_tree::BinaryTree, traits::binary_tree_traits::BinaryTreeTrait}, linked_list::{linked_list::LinkedList, traits::linked_list_traits::LinkedListTrait}, stack::{stack::Stack, traits::stack_traits::StackTrait}};
use crate::heap::heap::Heap;
use crate::heap::traits::heap_traits::HeapTrait;
use crate::queue::queue::Queue;
use crate::queue::traits::queue_traits::QueueTrait;

mod stack;
mod linked_list;
mod queue;
mod heap;
mod binary_tree;
mod hash_map;

fn main() {
    let mut stack: Stack<i128> = Stack::new();
    stack.push(10);
    stack.push(54);
    println!("{:?}", stack);

    let mut linkedlist:LinkedList<i128> = LinkedList::new();
    linkedlist.push(67);
    linkedlist.push(54);
    println!("{:?}", linkedlist);

    let mut queue:Queue<i128> = Queue::new();
    queue.enqueue(35);
    println!("{:?}", queue);

    let mut heap:Heap<i128, String> = Heap::new(true, 10);
    heap.push(16, "world".to_string());
    heap.push(10, "hello".to_string());
    println!("{:?}", heap);

    let mut binary_tree:BinaryTree<i128, &str> = BinaryTree::new();
    let _ = binary_tree.insert(1, "John", None);
    let _ = binary_tree.insert(2, "Bob", Some(1));
    let _ = binary_tree.insert(3, "Ana", Some(1));
    let _ = binary_tree.insert(4, "Sophie", Some(2));
    println!("{:?}", binary_tree);

    let mut hash_map:HashMap<&str, bool> = HashMap::new();
    hash_map.insert("Australia", true);
    hash_map.insert("United States", true);
    hash_map.insert("Uruguay", true);
    hash_map.insert("Antartica", false);
    println!("{:?}", hash_map);
}
