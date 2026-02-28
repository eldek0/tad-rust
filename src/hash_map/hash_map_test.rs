#[cfg(test)]
mod test{
    use crate::hash_map::{hash_map::HashMap, traits::hash_map_traits::HashMapTrait};

    #[test]
    fn create(){
        let new:HashMap<i16, i16> = HashMap::new(10);
        assert_eq!(0, new.size());
        assert_eq!(10, new.capacity());
    }

    #[test]
    fn insert(){
        let mut new:HashMap<i16, i16> = HashMap::new(10);
        new.insert(1, 10);
        new.insert(2, 12);
        println!("{}", new.size());

        assert_eq!(2, new.size());
        assert_eq!(10, new.capacity());
    }

    #[test]
    fn delete(){
        let mut new:HashMap<i16, i16> = HashMap::new(10);
        new.insert(1, 15);
        new.insert(2, 23);
        new.remove(&1).ok();

        assert_eq!(1, new.size());
        assert_eq!(10, new.capacity());
    }

    #[test]
    fn get(){
        let mut new:HashMap<i16, i16> = HashMap::new(10);
        new.insert(1, 15);
        new.insert(2, 23);

        assert_eq!(2, new.size());
        assert_eq!(10, new.capacity());
        assert_eq!(&15, new.get(&1).unwrap());
    }

    #[test]
    fn get_mut(){
        let mut new:HashMap<i16, i16> = HashMap::new(10);
        new.insert(1, 15);
        new.insert(2, 23);

        assert_eq!(2, new.size());
        assert_eq!(10, new.capacity());
        assert_eq!(&mut 15, new.get_mut(&1).unwrap());
    }

    #[test]
    fn overwrite_value() {
        let mut map: HashMap<i16, i16> = HashMap::new(10);

        map.insert(1, 10);
        map.insert(1, 20);

        assert_eq!(1, map.size());
        assert_eq!(&20, map.get(&1).unwrap());
    }

    #[test]
    fn get_nonexistent() {
        let map: HashMap<i16, i16> = HashMap::new(10);

        assert!(map.get(&99).is_err());
    }

    #[test]
    fn remove_nonexistent() {
        let mut map: HashMap<i16, i16> = HashMap::new(10);

        assert!(map.remove(&50).is_err());
    }

    #[test]
    fn contains_key() {
        let mut map: HashMap<i16, i16> = HashMap::new(10);

        map.insert(1, 10);

        assert!(map.contains_key(&1));
        assert!(!map.contains_key(&2));
    }

    #[test]
    fn collision_test() {
        let mut map: HashMap<i16, i16> = HashMap::new(5);

        map.insert(1, 10); // 1 % 5 == 1
        map.insert(6, 20); // 6 % 5 == 1

        assert_eq!(2, map.size());

        assert_eq!(&10, map.get(&1).unwrap());
        assert_eq!(&20, map.get(&6).unwrap());
    }

    #[test]
    fn for_loop_syntax() {
        let mut hash_map: HashMap<i32, i32> = HashMap::new(10);
        hash_map.insert(1, 10);
        hash_map.insert(2, 11);
        hash_map.insert(3, 12);

        let mut sum = 0;
        for (k, v) in hash_map.iter() {
            sum += *v;
        }

        assert_eq!(sum, 33);
    }
}