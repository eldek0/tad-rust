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
}