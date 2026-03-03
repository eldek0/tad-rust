use eldek_tad::stack::{stack::Stack, traits::stack_traits::StackTrait};

fn main(){
    let mut stack: Stack<i128> = Stack::new();
    stack.push(10);
    stack.push(54);
    println!("{:?}", stack);
}