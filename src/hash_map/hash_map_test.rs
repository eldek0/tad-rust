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
    fn put(){
        let mut new:HashMap<i16, i16> = HashMap::new(10);
        assert!(new.put(1, 10).is_ok());
        assert!(new.put(2, 12).is_ok());
        println!("{}", new.size());

        assert_eq!(2, new.size());
        assert_eq!(10, new.capacity());
    }

    #[test]
    fn delete(){
        let mut new:HashMap<i16, i16> = HashMap::new(10);
        assert!(new.put(1, 15).is_ok());
        assert!(new.put(2, 23).is_ok());
        assert!(new.remove(&1).is_ok());

        assert_eq!(1, new.size());
        assert_eq!(10, new.capacity());
    }

    #[test]
    fn get(){
        let mut new:HashMap<i16, i16> = HashMap::new(10);
        assert!(new.put(1, 15).is_ok());
        assert!(new.put(2, 23).is_ok());

        assert_eq!(2, new.size());
        assert_eq!(10, new.capacity());
        assert_eq!((&1, &15), new.get(&1).unwrap());
    }

    #[test]
    fn get_mut(){
        let mut new:HashMap<i16, i16> = HashMap::new(10);
        assert!(new.put(1, 15).is_ok());
        assert!(new.put(2, 23).is_ok());

        assert_eq!(2, new.size());
        assert_eq!(10, new.capacity());
        assert_eq!((&1, &mut 15), new.get_mut(&1).unwrap());
    }

    #[test]
    fn overwrite_value() {
        let mut map: HashMap<i16, i16> = HashMap::new(10);

        assert!(map.put(1, 10).is_ok());
        assert!(map.put(1, 20).is_ok());

        assert_eq!(1, map.size());
        assert_eq!((&1, &20), map.get(&1).unwrap());
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
    fn contains() {
        let mut map: HashMap<i16, i16> = HashMap::new(10);

        map.put(1, 10).unwrap();

        assert!(map.contains(&1));
        assert!(!map.contains(&2));
    }

    #[test]
    fn collision_test() {
        let mut map: HashMap<i16, i16> = HashMap::new(5);

        map.put(1, 10).unwrap(); // 1 % 5 == 1
        map.put(6, 20).unwrap(); // 6 % 5 == 1

        assert_eq!(2, map.size());

        assert_eq!((&1, &10), map.get(&1).unwrap());
        assert_eq!((&6, &20), map.get(&6).unwrap());
    }
}