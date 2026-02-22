#[cfg(test)]
mod test{
    use crate::linkedlist::{self, linkedlist::Linkedlist, traits::linkedlist_traits::LinkedlistTrait};

    #[test]
    fn create(){
        let linkedlist: Linkedlist<i32> = Linkedlist::new();
    }

    #[test]
    fn new_from(){
        let linkedlist: Linkedlist<i32> = Linkedlist::new_from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        assert_eq!(9, linkedlist.size());
    }

    #[test]
    fn add_elements(){
        let mut linkedlist: Linkedlist<i32> = Linkedlist::new();
        linkedlist.add(10);
        linkedlist.add(20);

        assert_eq!(2, linkedlist.size());
    }

    #[test]
    fn remove_elements(){
        let mut linkedlist: Linkedlist<i32> = Linkedlist::new();
        linkedlist.add(10);
        linkedlist.add(20);

        linkedlist.remove(1);

        assert_eq!(1, linkedlist.size());
    }

    #[test]
    #[should_panic]
    fn remove_out_of_bounds_panic(){
        let mut linkedlist: Linkedlist<i32> = Linkedlist::new();
        linkedlist.add(10);
        linkedlist.remove(5).unwrap();
    }

    #[test]
    #[should_panic]
    fn get_out_of_bounds_panic(){
        let mut linkedlist: Linkedlist<i32> = Linkedlist::new();
        linkedlist.add(10);
        linkedlist.get(5).unwrap();
    }

    #[test]
    #[should_panic]
    fn remove_empty_list(){
        let mut linkedlist: Linkedlist<i32> = Linkedlist::new();
        linkedlist.remove(0).unwrap();
    }

    #[test]
    #[should_panic]
    fn get_empty_list(){
        let linkedlist: Linkedlist<i32> = Linkedlist::new();
        linkedlist.get(0).unwrap();
    }

    #[test]
    fn remove_all(){
        let mut linkedlist: Linkedlist<i32> = Linkedlist::new();
        linkedlist.add(10);
        linkedlist.remove(0).unwrap();
        assert_eq!(0, linkedlist.size());
    }

    #[test]
    fn print_list(){
        let mut linkedlist: Linkedlist<i32> = Linkedlist::new();
        linkedlist.add(10);
        linkedlist.add(20);
        linkedlist.add(30);

        println!("{}", linkedlist);
        
        assert_eq!(10, *linkedlist.get(0).unwrap());
        assert_eq!(20, *linkedlist.get(1).unwrap());
        assert_eq!(30, *linkedlist.get(2).unwrap());
    }
}