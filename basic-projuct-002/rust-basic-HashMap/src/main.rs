use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("hello"), 10);
    map.insert(String::from("world"), 20);
    for (key, value) in &map {
        println!("key={},value={} ", key, value);
    }
    
    let  keys = vec![String::from("red"), String::from("green")];
    let  values = vec![10, 20];
    let mut maps: HashMap<_, _> = keys.iter().zip(values.iter()).collect();

    let key = String::from("red");
    if let Some(value) = maps.get(&key){
        println!("value={}", value);
    }else { 
        println!("value is none");
    }
    /*let value = maps.get(&key);
    if value.is_some(){
        println!("value={}", value.unwrap());
    }else { 
        println!("value is none");
    }*/
    
    let key = String::from("green");
    let value = maps.get(&key);
    match value {
        Some(value) => println!("value={}", value),
        None => println!("value is non"),
    }
    let text  = "hello world hello rust hello rust";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    for(key, value) in map{
        println!("key={},value={}", key, value);
    }
}
