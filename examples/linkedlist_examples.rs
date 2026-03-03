use eldek_tad::linked_list::{linked_list::LinkedList, traits::linked_list_traits::LinkedListTrait};

fn main(){
    let mut linkedlist:LinkedList<i128> = LinkedList::new();
    linkedlist.push(67);
    linkedlist.push(54);
    println!("{:?}", linkedlist);
}