#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{linkedlist::{linkedlist::Linkedlist, traits::linkedlist_traits::LinkedlistTrait}, stack::{stack::Stack, traits::stack_traits::StackTrait}};
use crate::queue::queue::Queue;
use crate::queue::traits::queue_traits::QueueTrait;

mod stack;
mod linkedlist;
mod queue;

fn main() {
    let stack: Stack<i128> = Stack::new();
    println!("{}", stack);

    let linkedlist:Linkedlist<i128> = Linkedlist::new();
    println!("{}", linkedlist);

    let queue:Queue<i128> = Queue::new();
    println!("{}", queue);
}
