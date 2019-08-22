use std:: collections::HashMap;

// 统计一些文本中每一个单词分别出现了多少次
fn count_word() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("count {:?}", map);
}

fn vector_example() {
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
}

fn hashmap_example() {
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
}

fn string_example() {
    let mut s = String::from("str");
    s.push_str("add");
    println!("s {}", s);
    for c in s.chars() {
        println!("char {}", c);
    }
}

fn overlap() {
    let a = [1, 2, 3];
    let b = [1, 2, 3, 4];

    let c: Vec<i32> = a.iter().zip(&b).map(|(a, b)| a & b).collect();
    println!("overlap {:?}", c);    
}

fn bunch_of_numbers() -> Vec<u32> {
    (0..10).collect()
}

fn new_vector_example() {
    let nums = bunch_of_numbers();

    match nums.last() {
        Some(&0) => println!("Last number is zero"),
        Some(n) => println!("Last number is {}", n),
        None => println!("There are no numbers"),
    }
}

fn slices_example() {
    let mut foo = [0u8; 5];
    foo[1] = 1;
    foo[2] = 2;

    let bar = &foo[..3];
    println!("{:?}", bar);
}

fn main() {
    vector_example();
    hashmap_example();
    string_example();
    count_word();
    overlap();
    new_vector_example();
    slices_example();
}