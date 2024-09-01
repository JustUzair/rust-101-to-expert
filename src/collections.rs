use std::collections::{HashMap, HashSet};

pub fn test_basic_hashmap() -> HashMap<String, f32> {
    let mut stock_list: HashMap<String, f32> = HashMap::new();
    stock_list.insert("Apple".to_string(), 10.0);
    stock_list.insert("Banana".to_string(), 5.0);
    stock_list.insert("Orange".to_string(), 7.0);
    stock_list.insert("Pineapple".to_string(), 15.0);
    stock_list.insert("Mango".to_string(), 20.0);

    println!("{:?}", stock_list.len());
    println!("{:?}", stock_list.get_key_value("Apple"));
    println!("is Empty : {:?}", stock_list.is_empty());
    println!("Remove : {:?}", stock_list.remove("Banana"));
    return stock_list;
}

pub fn test_basic_hashset() -> HashSet<String> {
    let mut stock_list: HashSet<String> = HashSet::new();
    stock_list.insert("Apple".to_string());
    stock_list.insert("Banana".to_string());
    stock_list.insert("Orange".to_string());
    stock_list.insert("Pineapple".to_string());
    stock_list.insert("Mango".to_string());

    println!("{:?}", stock_list.len());
    println!("{:?}", stock_list.get("Apple"));
    println!("is Empty : {:?}", stock_list.is_empty());
    println!("Remove : {:?}", stock_list.remove("Banana"));
    return stock_list;
}
