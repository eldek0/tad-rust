#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{binary_tree::{binary_tree::BinaryTree, traits::binary_tree_traits::BinaryTreeTrait}, linkedlist::{linkedlist::Linkedlist, traits::linkedlist_traits::LinkedlistTrait}, stack::{stack::Stack, traits::stack_traits::StackTrait}};
use crate::heap::heap::Heap;
use crate::heap::traits::heap_traits::HeapTrait;
use crate::queue::queue::Queue;
use crate::queue::traits::queue_traits::QueueTrait;

mod stack;
mod linkedlist;
mod queue;
mod heap;
mod binary_tree;

fn main() {
    let mut stack: Stack<i128> = Stack::new();
    stack.push(10);
    stack.push(54);
    println!("{:?}", stack);

    let mut linkedlist:Linkedlist<i128> = Linkedlist::new();
    linkedlist.add(67);
    linkedlist.add(54);
    println!("{:?}", linkedlist);

    let mut queue:Queue<i128> = Queue::new();
    queue.push(35);
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
}
