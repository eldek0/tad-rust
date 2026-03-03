use eldek_tad::hash_map::{hash_map::HashMap, traits::hash_map_traits::HashMapTrait};

fn main(){
    let mut hash_map:HashMap<&str, bool> = HashMap::new(10);
    hash_map.insert("Australia", true);
    hash_map.insert("United States", true);
    hash_map.insert("Uruguay", true);
    hash_map.insert("Antartica", false);
    println!("{:?}", hash_map);
}