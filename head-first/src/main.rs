//use std::cell::Cell;
//
//fn foo(cell: &Cell<u32>) {
//    let value = cell.get();
//    cell.set(value * 2);
//}

//struct Foo {
//    x: u32,
//}
//
//fn print_foo(foo: &Foo) {
//    println!("x={}", foo.x);
//}

//fn change_foo(foo: &Foo) {
//    foo.x = foo.x *2;
//}

//fn change_foo(foo: &mut Foo) {
//    foo.x = foo.x * 2;
//}

//use std::cell::UnsafeCell;
//use std::cell::Cell;

//fn main() {

//    let mut s = String::from("hello");
//    let r1 = &mut s;
//    let r2 = &mut s; // 可变引用，不能有别名
//    println!("{}, {}", r1, r2);

//    let x = 1;
//    let y = &mut x; // 当有一个不可变值时，不能可变的借用它

//    let mut data = 1_i32;
//    let p : &i32 = &data;
//    data = 10;
//    println!("{}", *p);

//    let data : Cell<i32> = Cell::new(1);
//    let p = &data;
//    data.set(10);
//    println!("{}", p.get());
//
//    p.set(20);
//    println!("{:?}", data);

//}


//    let cell = Cell::new(0);
//    let value = cell.get();
//    let new_value = cell.get() + 1;
//    foo(&cell);
//    cell.set(new_value); // oops, we clobbered the work done by foo

// use std::thread;
// use std::time::Duration;

// fn main() {
//     let t = thread::Builder::new()
//         .name("child1".to_string())
//         .spawn(move || {
//             println!("enter child thread.");
//             thread::park();
//             println!("resume child thread");
//         }).unwrap();
//     println!("spawn a thread");
//     thread::sleep(Duration::new(5,0));
//     t.thread().unpark();
//     t.join();
//     println!("child thread finished");
// }
extern crate ring;

use ring::digest::{Algorithm, SHA512};
use merkle_tree::MerkleTree;

static ALGO: &'static Algorithm = &SHA512;

fn main() {
    let values = vec!["one", "two", "three", "four"];
    let tree = MerkleTree::new(&values, ALGO);
    let proof = tree.build_proof(&"one");
    let vec = proof.unwrap();
    tree.validate(&vec);
}


