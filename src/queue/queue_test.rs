use crate::stack::stack::Stack;

#[cfg(test)]
mod test{
    use crate::queue::queue::Queue;
    use crate::queue::traits::queue_traits::QueueTrait;

    #[test]
    fn create(){
        let queue:Queue<i16> = Queue::new();
    }

    #[test]
    fn push(){
        let mut queue = Queue::new();
        queue.push(1);
        assert_eq!(1, queue.peek().unwrap());
    }

    #[test]
    fn pop(){
        let mut queue = Queue::new();
        queue.push(10);
        queue.push(2);
        assert_eq!(10, queue.pop().unwrap());
        assert_eq!(1, queue.size());
    }

    #[test]
    #[should_panic]
    fn pop_empty_panic(){
        let mut queue: Queue<i16> = Queue::new();
        let _ = queue.pop().unwrap();
    }

    #[test]
    #[should_panic]
    fn peek_empty_panic(){
        let queue: Queue<i16> = Queue::new();
        let _ = queue.peek().unwrap();
    }
}