#[cfg(test)]
mod test{
    use crate::queue::queue::Queue;
    use crate::queue::traits::queue_traits::QueueTrait;

    #[test]
    fn create(){
        let queue:Queue<i16> = Queue::new();
    }

    #[test]
    fn create_from_vec(){
        let queue:Queue<i16> = Queue::from_vec(vec![1, 2, 3]);
        assert_eq!(3, queue.size())
    }

    #[test]
    fn enqueue(){
        let mut queue = Queue::new();
        queue.enqueue(1);
        assert_eq!(1, queue.size());
    }

    #[test]
    fn peek(){
        let mut queue = Queue::new();
        queue.enqueue(1);
        assert_eq!(&1, queue.peek().unwrap());
    }

    #[test]
    fn peek_mut(){
        let mut queue = Queue::new();
        queue.enqueue(1);
        assert_eq!(&1, queue.peek_mut().unwrap());
    }

    #[test]
    fn dequeue(){
        let mut queue = Queue::new();
        queue.enqueue(10);
        queue.enqueue(2);
        assert_eq!(10, queue.dequeue().unwrap());
        assert_eq!(1, queue.size());
    }

    #[test]
    fn dequeue_empty(){
        let mut queue: Queue<i16> = Queue::new();
        assert!(queue.dequeue().is_err());
    }

    #[test]
    fn peek_empty_panic(){
        let queue: Queue<i16> = Queue::new();
        assert!(queue.peek().is_err());
    }

    #[test]
    fn into_iter_order(){
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        let collected: Vec<i32> = queue.into_iter().collect();
        assert_eq!(vec![1, 2, 3], collected);
    }

    #[test]
    fn into_iter_empty(){
        let queue: Queue<i32> = Queue::new();
        let collected: Vec<i32> = queue.into_iter().collect();
        assert_eq!(Vec::<i32>::new(), collected);
    }

    #[test]
    fn into_iter_consumes(){
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);

        let mut iter = queue.into_iter();
        assert_eq!(Some(1), iter.next());
        assert_eq!(Some(2), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn into_iter_sum(){
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        let sum: i32 = queue.into_iter().sum();
        assert_eq!(6, sum);
    }

    #[test]
    fn into_iter_consumes_list() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        let mut iter = queue.into_iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }
}