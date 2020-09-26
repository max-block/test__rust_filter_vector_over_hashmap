use std::collections::HashMap;

fn main() {
    let list = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let mut map: HashMap<String, bool> = HashMap::new();
    map.insert("b".to_string(), true);

    let res: Vec<String> = list.into_iter().filter(|x| !map.contains_key(x)).collect();
    println!("{:?}", res); // ["a", "c"]
}
