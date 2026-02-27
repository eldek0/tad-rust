#[cfg(test)]
mod test {
    use crate::binary_tree::binary_search_tree::BinarySearchTree;
    use crate::binary_tree::traits::binary_tree_traits::BinaryTreeTrait;

    #[test]
    fn create() {
        let tree: BinarySearchTree<i8, i8> = BinarySearchTree::new();
    }

    #[test]
    fn insert() {
        let mut tree: BinarySearchTree<i8, i8> = BinarySearchTree::new();
        assert!(tree.insert(4, 40).is_ok());
        assert!(tree.insert(2, 20).is_ok());
        assert!(tree.insert(6, 60).is_ok());

        assert_eq!(3, tree.size());
    }

    #[test]
    fn insert_orders_correctly() {
        let mut tree: BinarySearchTree<i16, i16> = BinarySearchTree::new();
        assert!(tree.insert(4, 40).is_ok());
        assert!(tree.insert(2, 20).is_ok());
        assert!(tree.insert(6, 60).is_ok());
        assert!(tree.insert(1, 10).is_ok());
        assert!(tree.insert(3, 30).is_ok());

        // Verifica que esten en el lugar correcto
        assert_eq!((&1, &10), tree.find(&1).unwrap());
        assert_eq!((&3, &30), tree.find(&3).unwrap());
        assert_eq!((&6, &60), tree.find(&6).unwrap());
    }

    #[test]
    fn insert_duplicate_key() {
        let mut tree: BinarySearchTree<i32, i32> = BinarySearchTree::new();
        assert!(tree.insert(1, 10).is_ok());
        assert!(tree.insert(1, 999).is_err());
        assert_eq!(1, tree.size());
    }

    #[test]
    fn delete() {
        let mut tree: BinarySearchTree<i16, i16> = BinarySearchTree::new();
        assert!(tree.insert(4, 40).is_ok());
        assert!(tree.insert(2, 20).is_ok());
        assert!(tree.insert(6, 60).is_ok());
        assert!(tree.delete(&2).is_ok());

        assert_eq!(2, tree.size());
    }

    #[test]
    fn delete_cascade() {
        let mut tree: BinarySearchTree<i16, i16> = BinarySearchTree::new();
        assert!(tree.insert(4, 40).is_ok());
        assert!(tree.insert(2, 20).is_ok());
        assert!(tree.insert(1, 10).is_ok());
        assert!(tree.insert(3, 30).is_ok());
        assert!(tree.delete(&2).is_ok());

        assert_eq!(1, tree.size());
    }

    #[test]
    fn find() {
        let mut tree: BinarySearchTree<i16, i16> = BinarySearchTree::new();
        assert!(tree.insert(4, 40).is_ok());
        assert!(tree.insert(2, 20).is_ok());
        assert!(tree.insert(6, 60).is_ok());

        assert_eq!((&2, &20), tree.find(&2).unwrap());
    }

    #[test]
    fn find_mut() {
        let mut tree: BinarySearchTree<i16, i16> = BinarySearchTree::new();
        assert!(tree.insert(4, 40).is_ok());
        assert!(tree.insert(2, 20).is_ok());

        assert_eq!((&2, &mut 20), tree.find_mut(&2).unwrap());
    }

    #[test]
    fn find_empty_tree() {
        let tree: BinarySearchTree<i32, i32> = BinarySearchTree::new();
        assert!(tree.find(&1).is_err());
    }

    #[test]
    fn delete_empty_tree() {
        let mut tree: BinarySearchTree<i32, i32> = BinarySearchTree::new();
        assert!(tree.delete(&1).is_err());
        assert_eq!(0, tree.size());
    }

    #[test]
    fn find_non_existent_key() {
        let mut tree: BinarySearchTree<i32, i32> = BinarySearchTree::new();
        tree.insert(4, 40).unwrap();
        assert!(tree.find(&999).is_err());
    }

    #[test]
    fn delete_non_existent_key() {
        let mut tree: BinarySearchTree<i32, i32> = BinarySearchTree::new();
        tree.insert(4, 40).unwrap();
        assert!(tree.delete(&999).is_err());
        assert_eq!(1, tree.size());
    }

    #[test]
    fn count_leaves() {
        let mut tree: BinarySearchTree<i16, i16> = BinarySearchTree::new();
        assert!(tree.insert(4, 40).is_ok());
        assert!(tree.insert(2, 20).is_ok());
        assert!(tree.insert(6, 60).is_ok());
        assert!(tree.insert(1, 10).is_ok());
        assert!(tree.insert(3, 30).is_ok());

        assert_eq!(3, tree.count_leaves());
    }

    #[test]
    fn empty_tree_count_leaves() {
        let tree: BinarySearchTree<i32, i32> = BinarySearchTree::new();
        assert_eq!(0, tree.count_leaves());
    }
}