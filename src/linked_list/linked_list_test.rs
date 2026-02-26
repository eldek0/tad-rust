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
        linkedlist.push(10);
        linkedlist.push(20);

        assert_eq!(2, linkedlist.size());
    }

    #[test]
    fn remove_elements(){
        let mut linkedlist: LinkedList<i32> = LinkedList::new();
        linkedlist.push(10);
        linkedlist.push(20);

        assert!(linkedlist.remove(1).is_ok());

        assert_eq!(1, linkedlist.size());
    }

    #[test]
    fn remove_out_of_bounds_panic(){
        let mut linkedlist: LinkedList<i32> = LinkedList::new();
        linkedlist.push(10);
        assert!(linkedlist.remove(10).is_err());
        assert_eq!(1, linkedlist.size());
    }

    #[test]
    fn get_out_of_bounds_panic(){
        let mut linkedlist: LinkedList<i32> = LinkedList::new();
        linkedlist.push(10);
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
        linkedlist.push(10);
        linkedlist.remove(0).unwrap();
        assert_eq!(0, linkedlist.size());
    }

    #[test]
    fn print_list(){
        let mut linkedlist: LinkedList<i32> = LinkedList::new();
        linkedlist.push(10);
        linkedlist.push(20);
        linkedlist.push(30);

        println!("{:?}", linkedlist);
        
        assert_eq!(10, *linkedlist.get(0).unwrap());
        assert_eq!(20, *linkedlist.get(1).unwrap());
        assert_eq!(30, *linkedlist.get(2).unwrap());
    }
    #[test]
    fn remove_last_element() {
        let mut linkedlist: LinkedList<i32> = LinkedList::new();
        linkedlist.push(10);
        linkedlist.push(20);
        linkedlist.push(30);

        linkedlist.remove(2).unwrap();

        assert_eq!(2, linkedlist.size());
        assert_eq!(10, *linkedlist.get(0).unwrap());
        assert_eq!(20, *linkedlist.get(1).unwrap());
    }
    #[test]
    fn remove_middle_element() {
        let mut linkedlist: LinkedList<i32> = LinkedList::new();
        linkedlist.push(10);
        linkedlist.push(20);
        linkedlist.push(30);

        linkedlist.remove(1).unwrap();

        assert_eq!(2, linkedlist.size());
        assert_eq!(10, *linkedlist.get(0).unwrap());
        assert_eq!(30, *linkedlist.get(1).unwrap());
    }

    #[test]
    fn into_iter_consumes_list() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_by_reference() {
        let mut list = LinkedList::new();
        list.push(10);
        list.push(20);
        list.push(30);

        let mut iter = (&list).into_iter();

        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&20));
        assert_eq!(iter.next(), Some(&30));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut_modifies_values() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        for x in &mut list {
            *x *= 2;
        }

        let collected: Vec<_> = list.into_iter().collect();
        assert_eq!(collected, vec![2, 4, 6]);
    }

    #[test]
    fn from_iterator_collect() {
        let list: LinkedList<i32> = vec![5, 6, 7].into_iter().collect();

        let collected: Vec<_> = list.into_iter().collect();
        assert_eq!(collected, vec![5, 6, 7]);
    }

    #[test]
    fn empty_list_iter() {
        let list: LinkedList<i32> = LinkedList::new();

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn for_loop_syntax() {
        let list: LinkedList<i32> = vec![1, 2, 3].into_iter().collect();

        let mut sum = 0;
        for x in &list {
            sum += *x;
        }

        assert_eq!(sum, 6);
    }

}