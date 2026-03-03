use eldek_tad::binary_tree::{binary_tree::BinaryTree, traits::binary_tree_traits::BinaryTreeTrait};

fn main(){
    let mut binary_tree:BinaryTree<i128, &str> = BinaryTree::new();
    binary_tree.insert(1, "John", None).ok();
    binary_tree.insert(2, "Bob", Some(1)).ok();
    binary_tree.insert(3, "Ana", Some(1)).ok();
    binary_tree.insert(4, "Sophie", Some(2)).ok();
    println!("{:?}", binary_tree);
}