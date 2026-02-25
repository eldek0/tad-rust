#[cfg(test)]
mod test{
    use crate::heap::heap::Heap;
    use crate::heap::traits::heap_traits::HeapTrait;

    #[test]
    fn create(){
        let heap:Heap<i8, String> = Heap::new(true, 10);
    }

    #[test]
    fn push_max(){
        let mut heap:Heap<i8, &str> = Heap::new(false, 10);

        heap.push(6, "Pasta");
        heap.push(10, "Pizza");
        assert_eq!(heap.peek().unwrap(), (&10, &"Pizza"));
        assert_eq!(heap.size(), 2);
    }

    #[test]
    fn push_min(){
        let mut heap:Heap<i8, &str> = Heap::new(true, 10);

        heap.push(6, "Pasta");
        heap.push(10, "Pizza");
        assert_eq!(heap.peek().unwrap(), (&6, &"Pasta"));
        assert_eq!(heap.size(), 2);
    }

    #[test]
    fn peek(){
        let mut heap:Heap<i8, &str> = Heap::new(true, 10);

        heap.push(6, "Pasta");
        heap.push(10, "Pizza");
        assert_eq!(heap.peek().unwrap(), (&6, &"Pasta"));
    }

    #[test]
    fn peek_mut(){
        let mut heap:Heap<i8, &str> = Heap::new(true, 10);

        heap.push(6, "Pasta");
        heap.push(10, "Pizza");
        assert_eq!(heap.peek_mut().unwrap(), (&6, &mut "Pasta"));
    }

    #[test]
    fn pop_max(){
        let mut heap:Heap<i8, &str> = Heap::new(true, 10);

        heap.push(10, "Hamburger");
        heap.push(6, "Pasta");
        assert_eq!(heap.pop().unwrap(), (6, "Pasta"));
        assert_eq!(heap.size(), 1);
    }
    #[test]
    fn pop_min(){
        let mut heap:Heap<i8, &str> = Heap::new(false, 10);

        heap.push(10, "Hamburger");
        heap.push(6, "Pasta");
        assert_eq!(heap.pop().unwrap(), (10, "Hamburger"));
        assert_eq!(heap.size(), 1);
    }
    #[test]
    fn resize_test(){
        let mut heap:Heap<i8, &str> = Heap::new(false, 2);

        heap.push(10, "Pizza");
        heap.push(9, "Sushi");
        heap.push(10, "Tacos");
        heap.push(10, "Milanesa");
        heap.push(3, "Ensalada");
        heap.push(10, "Ramen");
        heap.push(10, "Empanadas");
        heap.push(10, "Asado");
        heap.push(7, "Sandwich");
        heap.push(7, "Hot Dog");
        heap.push(-3, "Salad");

        assert_eq!(heap.size(), 11);
    }
    #[test]
    fn pop_empty() {
        let mut heap: Heap<i8, &str> = Heap::new(true, 10);
        assert!(heap.pop().is_err());
    }
    #[test]
    fn peek_empty() {
        let heap: Heap<i8, &str> = Heap::new(true, 10);
        assert!(heap.peek().is_err());
    }
    #[test]
    fn pop_order_max() {
        let mut heap: Heap<i32, &str> = Heap::new(false, 10);

        heap.push(5, "A");
        heap.push(10, "B");
        heap.push(3, "C");
        heap.push(8, "D");

        assert_eq!(heap.pop().unwrap().0, 10);
        assert_eq!(heap.pop().unwrap().0, 8);
        assert_eq!(heap.pop().unwrap().0, 5);
        assert_eq!(heap.pop().unwrap().0, 3);
    }
    #[test]
    fn same_priority() {
        let mut heap: Heap<i8, &str> = Heap::new(false, 10);

        heap.push(10, "A");
        heap.push(10, "B");
        heap.push(10, "C");

        assert_eq!(heap.size(), 3);
    }
    #[test]
    fn reuse_after_empty() {
        let mut heap: Heap<i8, &str> = Heap::new(false, 10);

        heap.push(5, "A");
        assert!(heap.pop().is_ok());
        heap.push(8, "B");

        assert_eq!(heap.peek().unwrap().0, &8);
    }
}