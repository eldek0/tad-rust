#[cfg(test)]
mod test{
    use crate::stack::{stack::Stack, traits::stack_traits::StackTrait};

    #[test]
    fn create(){
        let stack: Stack<i32> = Stack::new();
    }

    #[test]
    fn create_from_vec(){
        let stack:Stack<i16> = Stack::from_vec(vec![1, 2, 3]);
        assert_eq!(3, stack.size())
    }

    #[test]
    fn push(){
        let mut stack = Stack::new();
        stack.push(1);
        assert_eq!(1, stack.size());
    }

    #[test]
    fn peek(){
        let mut stack=Stack::new();
        stack.push(1);
        assert_eq!(&1, stack.peek().unwrap());
    }

    #[test]
    fn peek_mut(){
        let mut stack=Stack::new();
        stack.push(1);
        assert_eq!(&1, stack.peek_mut().unwrap());
    }

    #[test]
    fn pop(){
        let mut stack = Stack::new();
        stack.push(10);
        stack.push(2);
        assert_eq!(2, stack.pop().unwrap());
        assert_eq!(1, stack.size());
    }

    #[test]
    fn pop_empty(){
        let mut stack: Stack<i16> = Stack::new();
        assert!(stack.pop().is_err());
    }

    #[test]
    fn peek_empty_panic(){
        let stack: Stack<i16> = Stack::new();
        assert!(stack.peek().is_err());
    }

    #[test]
    fn into_iter(){
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        
        let collected: Vec<i32> = stack.into_iter().collect();
        assert_eq!(vec![3, 2, 1], collected);
    }

    #[test]
    fn into_iter_empty(){
        let stack: Stack<i32> = Stack::new();
        let collected: Vec<i32> = stack.into_iter().collect();
        assert_eq!(Vec::<i32>::new(), collected);
    }

    #[test]
    fn into_iter_sum(){
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        
        let sum: i32 = stack.into_iter().sum();
        assert_eq!(6, sum);
    }

    #[test]
    fn into_iter_count(){
        let mut stack = Stack::new();
        stack.push(10);
        stack.push(20);
        
        assert_eq!(2, stack.into_iter().count());
    }

    #[test]
    fn into_iter_order(){
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        let mut iter = stack.into_iter();
        assert_eq!(Some(3), iter.next()); // top first
        assert_eq!(Some(2), iter.next());
        assert_eq!(Some(1), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn into_iter_consumes_list() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        let mut iter = stack.into_iter();

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
}