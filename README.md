# tad-rust

A project for learning Abstract Data Types (ADTs) in Rust.
Heavily inspired by Java's implementations.

[![Crates.io](https://img.shields.io/crates/v/eldek-tad.svg)](https://crates.io/crates/eldek-tad)
[![Docs.rs](https://docs.rs/eldek-tad/badge.svg)](https://docs.rs/eldek-tad)

## Structures
- [x] Linked List
- [x] Stack
- [x] Queue
- [x] Heap
- [x] Binary Tree
- [x] HashMap
- [x] Graph
- [x] Binary Search Tree (BST)
- [x] Prefix Tree

## Usage
```toml
[dependencies]
eldek-tad = "0.9.1"
```

## Example
```rust
use eldek_tad::stack::stack::Stack;
use eldek_tad::stack::traits::stack_traits::StackTrait;

fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Ok(val) = stack.pop() {
        println!("{}", val); // 3, 2, 1
    }
}
```

## Documentation
https://docs.rs/eldek-tad