use std:: collections::HashMap;

fn main() {
    // vector
    let v: Vec<i32> = Vec::new();
    println!("vec size {}", v.len());
    let mut v = Vec::new();
    v.push(8);
    println!("after push vec size {}", v.len());
    let v = vec![1, 2];
    println!("vec get element {}", &v[0]);
    for i in &v {
        println!("vec each item {}", i);
    }

    //HashMap<K, V>
    let mut hmap = HashMap::new();
    hmap.insert("one", 1);
    hmap.insert("one", 2);
    for (key, value) in &hmap {
        println!("{}: {}", key, value);
    }
    hmap.entry("two").or_insert(3);
    println!("after insert hashmap {:?}", hmap);

    //字符串
    let mut s = String::from("string");
    s.push_str("add");
    println!("s {}", s);
    for c in s.chars() {
        println!("char {}", c);
    }
}