#[cfg(test)]
mod test{
    use crate::{linked_list::traits::linked_list_traits::LinkedListTrait, prefix_tree::{prefix_tree::PrefixTree, traits::prefix_tree_trait::PrefixTreeTrait}};

    #[test]
    fn search() {
        let mut tree = PrefixTree::new();
        tree.insert(vec!['h', 'e', 'l', 'l', 'o']);
        assert!(tree.search(vec!['h', 'e', 'l', 'l', 'o']));
    }

    #[test]
    fn search_string() {
        let mut tree:PrefixTree<char> = PrefixTree::new();
        tree.insert_string("hello");
        assert!(tree.search_string("hello"));
    }

    #[test]
    fn search_not_inserted() {
        let mut tree = PrefixTree::new();
        tree.insert(vec!['h', 'e', 'l', 'l', 'o']);
        assert!(!tree.search(vec!['h', 'e', 'l', 'l']));
    }

    #[test]
    fn search_empty_tree() {
        let tree: PrefixTree<char> = PrefixTree::new();
        assert!(!tree.search(vec!['a']));
    }

    #[test]
    fn insert_multiple_search_all() {
        let mut tree = PrefixTree::new();
        tree.insert(vec!['c', 'a', 't']);
        tree.insert(vec!['c', 'a', 'r']);
        tree.insert(vec!['d', 'o', 'g']);
        assert!(tree.search(vec!['c', 'a', 't']));
        assert!(tree.search(vec!['c', 'a', 'r']));
        assert!(tree.search(vec!['d', 'o', 'g']));
    }

    #[test]
    fn search_prefix_of_inserted() {
        let mut tree = PrefixTree::new();
        tree.insert(vec!['c', 'a', 't']);
        assert!(!tree.search(vec!['c', 'a']));
    }

    #[test]
    fn insert_empty_sequence() {
        let mut tree: PrefixTree<char> = PrefixTree::new();
        tree.insert(vec![]);
        assert!(tree.search(vec![]));
    }

    #[test]
    fn remove(){
        let mut tree= PrefixTree::new();
        tree.insert(vec![1, 2, 3]);
        tree.remove(vec![1, 2, 3]);
        assert!(!tree.search(vec![1, 2, 3]));
        assert_eq!(0, tree.size());
    }

    #[test]
    fn remove_string(){
        let mut tree:PrefixTree<char>= PrefixTree::new();
        tree.insert_string("Howdy");
        tree.remove_string("Howdy");
        assert_eq!(false, tree.search_string("Howdy"));
        assert_eq!(0, tree.size());
    }

    #[test]
    fn remove_failed(){
        let mut tree= PrefixTree::new();
        tree.insert(vec![1, 2, 3]);
        tree.remove(vec![1, 2, 4]);
        assert!(tree.search(vec![1, 2, 3]));
        assert_eq!(1, tree.size());
    }

    #[test]
    fn autocomplete_no_match() {
        let mut tree = PrefixTree::new();
        tree.insert(vec!['c', 'a', 't']);
        let results = tree.autocomplete(vec!['d']);
        assert_eq!(results.size(), 0);
    }

    #[test]
    fn autocomplete_exact_match() {
        let mut tree = PrefixTree::new();
        tree.insert(vec!['c', 'a', 't']);
        let results = tree.autocomplete(vec!['c', 'a', 't']);
        assert_eq!(results.size(), 1);
    }

    #[test]
    fn autocomplete_empty_prefix_returns_all() {
        let mut tree = PrefixTree::new();
        tree.insert(vec!['a']);
        tree.insert(vec!['b']);
        tree.insert(vec!['c']);
        let results = tree.autocomplete(vec![]);
        assert_eq!(results.size(), 3);
    }

    #[test]
    fn autocomplete_empty_tree() {
        let tree: PrefixTree<char> = PrefixTree::new();
        let results = tree.autocomplete(vec!['a']);
        assert_eq!(results.size(), 0);
    }

    #[test]
    fn autocomplete_count() {
        let mut tree = PrefixTree::new();
        tree.insert(vec!['c', 'a', 't']);
        tree.insert(vec!['c', 'a', 'r']);
        tree.insert(vec!['c', 'a', 'r', 'g', 'o']);
        let results = tree.autocomplete(vec!['c', 'a']);
        assert_eq!(results.size(), 3);
    }

    #[test]
    fn autocomplete_string() {
        let mut tree:PrefixTree<char> = PrefixTree::new();
        tree.insert_string("cat");
        tree.insert_string("car");
        tree.insert_string("cargo");
        let results = tree.autocomplete_string("ca");
        assert_eq!(true, results.contains(&String::from("cat")));
        assert_eq!(true, results.contains(&String::from("car")));
        assert_eq!(true, results.contains(&String::from("cargo")));
        assert_eq!(results.size(), 3);
    }
}