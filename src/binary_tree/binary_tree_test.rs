#[cfg(test)]
mod test{
    use crate::binary_tree::binary_tree::BinaryTree;
    use crate::binary_tree::traits::binary_tree_traits::BinaryTreeTrait;

    #[test]
    fn create(){
        let tree:BinaryTree<i8, i8> = BinaryTree::new();
    }

    #[test]
    fn insert(){
        let mut tree:BinaryTree<i8, i8> = BinaryTree::new();
        assert!(tree.insert(1, 2, None).is_ok());
        assert!(tree.insert(2, 2, Some(1)).is_ok());
        assert!(tree.insert(3, 65, Some(1)).is_ok());

        assert_eq!(3, tree.size());
    }

    #[test]
    fn delete(){
        let mut tree:BinaryTree<i16, i16> = BinaryTree::new();
        assert!(tree.insert(1, 10, None).is_ok());
        assert!(tree.insert(2, 50, Some(1)).is_ok());
        assert!(tree.delete(&2).is_ok());

        assert_eq!(1, tree.size());
    }

    #[test]
    fn delete_cascade(){
        let mut tree:BinaryTree<i16, i16> = BinaryTree::new();
        assert!(tree.insert(1, 10, None).is_ok());
        assert!(tree.insert(2, 50, Some(1)).is_ok());
        assert!(tree.insert(3, 96, Some(2)).is_ok());
        assert!(tree.delete(&2).is_ok());

        assert_eq!(1, tree.size());
    }

    #[test]
    fn find(){
        let mut tree:BinaryTree<i16, i16> = BinaryTree::new();
        assert!(tree.insert(1, 10, None).is_ok());
        assert!(tree.insert(2, 50, Some(1)).is_ok());
        assert!(tree.insert(3, 96, Some(2)).is_ok());

        assert_eq!((&2, &50), tree.find(&2).unwrap());
    }

    #[test]
    fn find_mut(){
        let mut tree:BinaryTree<i16, i16> = BinaryTree::new();
        assert!(tree.insert(1, 10, None).is_ok());
        assert!(tree.insert(2, 50, Some(1)).is_ok());
        assert!(tree.insert(3, 96, Some(2)).is_ok());

        assert_eq!((&2, &mut 50), tree.find_mut(&2).unwrap());
    }

    #[test]
    fn count_leaves(){
        let mut tree: BinaryTree<i16, i16> = BinaryTree::new();

        assert!(tree.insert(1, 10, None).is_ok());

        assert!(tree.insert(2, 20, Some(1)).is_ok());
        assert!(tree.insert(4, 40, Some(1)).is_ok());

        assert!(tree.insert(3, 30, Some(2)).is_ok());
        assert!(tree.insert(5, 50, Some(2)).is_ok());
        assert!(tree.insert(6, 60, Some(4)).is_ok());

        assert!(tree.insert(7, 70, Some(6)).is_ok());

        assert_eq!(3, tree.count_leaves());
    }

    #[test]
    fn find_empty_tree() {
        let tree: BinaryTree<i32, i32> = BinaryTree::new();
        assert!(tree.find(&1).is_err());
    }

    #[test]
    fn delete_empty_tree() {
        let mut tree: BinaryTree<i32, i32> = BinaryTree::new();
        assert!(tree.delete(&1).is_err());
        assert_eq!(0, tree.size());
    }

    #[test]
    fn insert_second_root_error() {
        let mut tree: BinaryTree<i32, i32> = BinaryTree::new();

        assert!(tree.insert(1, 10, None).is_ok());
        assert!(tree.insert(2, 20, None).is_err());
        assert_eq!(1, tree.size());
    }

    #[test]
    fn test_insert_invalid_parent() {
        let mut tree: BinaryTree<i32, i32> = BinaryTree::new();

        tree.insert(1, 10, None).unwrap();
        assert!(tree.insert(2, 20, Some(999)).is_err());
        assert_eq!(1, tree.size());
    }

    #[test]
    fn test_insert_duplicate_key() {
        let mut tree: BinaryTree<i32, i32> = BinaryTree::new();

        tree.insert(1, 10, None).unwrap();
        assert!(tree.insert(1, 999, Some(1)).is_err());
        assert_eq!(1, tree.size());
    }

    #[test]
    fn find_non_existent_key() {
        let mut tree: BinaryTree<i32, i32> = BinaryTree::new();

        tree.insert(1, 10, None).unwrap();
        assert!(tree.find(&999).is_err());
        assert_eq!(1, tree.size());
    }

    #[test]
    fn delete_non_existent_key() {
        let mut tree: BinaryTree<i32, i32> = BinaryTree::new();

        tree.insert(1, 10, None).unwrap();
        assert!(tree.delete(&999).is_err());
        assert_eq!(1, tree.size());
    }

    #[test]
    fn empty_tree_count_leaves() {
        let tree: BinaryTree<i32, i32> = BinaryTree::new();

        assert_eq!(tree.count_leaves(), 0);
    }


}