use eldek_tad::prefix_tree::{prefix_tree::PrefixTree, traits::prefix_tree_trait::PrefixTreeTrait};

fn main(){
    let mut prefix:PrefixTree<char> = PrefixTree::new();
    prefix.insert_string("Banana");
    prefix.insert_string("Banena");

    println!("{:?}", prefix);

    let autocomplete = prefix.autocomplete_string("Ban");
    
    println!("{:?}", autocomplete);
}