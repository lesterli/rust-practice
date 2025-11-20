use std::collections::HashMap;

// HashMap examples and a word counter
pub fn run() {
    println!("-- maps (HashMap) module --");
    hashmap_example();
    count_words();
}

fn hashmap_example() {
    let mut hmap = HashMap::new();
    hmap.insert("one", 1);
    hmap.insert("one", 2); // demonstrate overwrite
    for (key, value) in &hmap {
        println!("init hashmap {}: {}", key, value);
    }
    {
        let val = hmap.entry("two").or_insert(3);
        println!("insert {}", val);
    }
    println!("after insert hashmap {:?}", hmap);
}

fn count_words() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("word counts: {:?}", map);
}