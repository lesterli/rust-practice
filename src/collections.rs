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

    // HashMap<K, V>
    let mut hmap = HashMap::new();
    hmap.insert("one", 1);
    hmap.insert("one", 2);
    for (key, value) in &hmap {
        println!("init hashmap {}: {}", key, value);
    }
    {
        let val = hmap.entry("two").or_insert(3);
        println!("insert {}", val);
    }
    println!("after insert hashmap {:?}", hmap);

    // 字符串
    let mut s = String::from("string");
    s.push_str("add");
    println!("s {}", s);
    for c in s.chars() {
        println!("char {}", c);
    }

    // 统计一些文本中每一个单词分别出现了多少次
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("count {:?}", map);
}