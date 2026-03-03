use eldek_tad::heap::{heap::Heap, traits::heap_traits::HeapTrait};

fn main(){
    let mut heap:Heap<i128, String> = Heap::new(true, 10);
    heap.push(16, "world".to_string());
    heap.push(10, "hello".to_string());
    println!("{:?}", heap);
}