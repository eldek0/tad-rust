use eldek_tad::binary_tree::binary_search_tree::BinarySearchTree;

fn main(){
    let mut bst:BinarySearchTree<i32, i32> = BinarySearchTree::new();
    bst.insert(10, 1).ok();
    bst.insert(5, 2).ok();
    bst.insert(4, 5).ok();
    bst.insert(15, 54).ok();
    println!("{:?}", bst);
}