use eldek_tad::queue::{queue::Queue, traits::queue_traits::QueueTrait};

fn main(){
    let mut queue:Queue<i128> = Queue::new();
    queue.enqueue(35);
    println!("{:?}", queue);
}