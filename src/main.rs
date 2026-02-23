#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{linkedlist::{linkedlist::Linkedlist, traits::linkedlist_traits::LinkedlistTrait}, stack::{stack::Stack, traits::stack_traits::StackTrait}};

mod stack;
mod linkedlist;

fn main() {
    let stack: Stack<i128> = Stack::new();
    println!("{}", stack);

    let linkedlist:Linkedlist<i128> = Linkedlist::new();
    println!("{}", linkedlist);
}
