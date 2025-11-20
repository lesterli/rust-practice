// Vec examples and helpers
pub fn run() {
    println!("-- vectors module --");
    vector_example();
    overlap_example();
    new_vector_example();
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

fn overlap_example() {
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