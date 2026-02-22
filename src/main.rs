#![allow(dead_code)]
#![allow(unused_variables)] 

use crate::linkedlist::{linked_list::Linkedlist, traits::linked_list_traits::LinkedlistTrait};

mod linkedlist;

fn main() {
    let mut linkedlist: Linkedlist<i32> = Linkedlist::new();
    linkedlist.add(10);
    linkedlist.add(20);

    println!("Size: {:?}", linkedlist.size());
    println!("{}", linkedlist);
}
