#[cfg(test)]
mod stack_test{
    use crate::stack::{stack::Stack, traits::stack_traits::StackTrait};

    #[test]
    fn create(){
        let stack: Stack<i32> = Stack::new();
    }

    #[test]
    fn push(){
        let mut stack = Stack::new();
        stack.push(1);
        assert_eq!(1, stack.peek().unwrap());
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
    #[should_panic]
    fn pop_empty_panic(){
        let mut stack: Stack<i16> = Stack::new();
        let _ = stack.pop().unwrap();
    }

    #[test]
    #[should_panic]
    fn peek_empty_panic(){
        let stack: Stack<i16> = Stack::new();
        let _ = stack.peek().unwrap();
    }
}