#[cfg(test)]
mod test{
    use crate::linked_list::{linked_list::LinkedList, traits::linked_list_traits::LinkedListTrait};

    #[test]
    fn create(){
        let linkedlist: LinkedList<i32> = LinkedList::new();
    }

    #[test]
    fn new_from(){
        let linkedlist: LinkedList<i32> = LinkedList::new_from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        assert_eq!(9, linkedlist.size());
    }

    #[test]
    fn get(){
        let linked_list:LinkedList<i16> = LinkedList::new_from(vec![1, 2, 3]);

        assert_eq!(&2, linked_list.get(1).unwrap());
    }

    #[test]
    fn get_mut(){
        let mut linked_list:LinkedList<i16> = LinkedList::new_from(vec![1, 2, 3]);

        assert_eq!(&2, linked_list.get_mut(1).unwrap());
    }

    #[test]
    fn add_elements(){
        let mut linkedlist: LinkedList<i32> = LinkedList::new();
        linkedlist.add(10);
        linkedlist.add(20);

        assert_eq!(2, linkedlist.size());
    }

    #[test]
    fn remove_elements(){
        let mut linkedlist: LinkedList<i32> = LinkedList::new();
        linkedlist.add(10);
        linkedlist.add(20);

        assert!(linkedlist.remove(1).is_ok());

        assert_eq!(1, linkedlist.size());
    }

    #[test]
    fn remove_out_of_bounds_panic(){
        let mut linkedlist: LinkedList<i32> = LinkedList::new();
        linkedlist.add(10);
        assert!(linkedlist.remove(10).is_err());
        assert_eq!(1, linkedlist.size());
    }

    #[test]
    fn get_out_of_bounds_panic(){
        let mut linkedlist: LinkedList<i32> = LinkedList::new();
        linkedlist.add(10);
        assert!(linkedlist.remove(1).is_err());
    }

    #[test]
    fn remove_empty_list(){
        let mut linkedlist: LinkedList<i32> = LinkedList::new();
        assert!(linkedlist.remove(0).is_err());
    }

    #[test]
    fn get_empty_list(){
        let linkedlist: LinkedList<i32> = LinkedList::new();
        assert!(linkedlist.get(0).is_err());
    }

    #[test]
    fn remove_all(){
        let mut linkedlist: LinkedList<i32> = LinkedList::new();
        linkedlist.add(10);
        linkedlist.remove(0).unwrap();
        assert_eq!(0, linkedlist.size());
    }

    #[test]
    fn print_list(){
        let mut linkedlist: LinkedList<i32> = LinkedList::new();
        linkedlist.add(10);
        linkedlist.add(20);
        linkedlist.add(30);

        println!("{:?}", linkedlist);
        
        assert_eq!(10, *linkedlist.get(0).unwrap());
        assert_eq!(20, *linkedlist.get(1).unwrap());
        assert_eq!(30, *linkedlist.get(2).unwrap());
    }
    #[test]
    fn remove_last_element() {
        let mut linkedlist: LinkedList<i32> = LinkedList::new();
        linkedlist.add(10);
        linkedlist.add(20);
        linkedlist.add(30);

        linkedlist.remove(2).unwrap();

        assert_eq!(2, linkedlist.size());
        assert_eq!(10, *linkedlist.get(0).unwrap());
        assert_eq!(20, *linkedlist.get(1).unwrap());
    }
    #[test]
    fn remove_middle_element() {
        let mut linkedlist: LinkedList<i32> = LinkedList::new();
        linkedlist.add(10);
        linkedlist.add(20);
        linkedlist.add(30);

        linkedlist.remove(1).unwrap();

        assert_eq!(2, linkedlist.size());
        assert_eq!(10, *linkedlist.get(0).unwrap());
        assert_eq!(30, *linkedlist.get(1).unwrap());
    }

}