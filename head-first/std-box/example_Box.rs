use List::{Cons, Nil};

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn main() {
	let recursive_list: List<i32> = Cons(1, Box::new(Cons(2, Box::new(Nil))));
	println!("{:?}", recursive_list); // 打印出：Cons(1, Cons(2, Nil))
}