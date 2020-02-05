trait T {
    fn m(&self) -> u64;
}
  
struct S {
    i: u64
}
  
impl T for S {
    fn m(&self) -> u64 { self.i }
}

fn f(x: Box<dyn T>) {
    println!("{}", x.m())
}
  
fn main() {
    let s = S{i : 100};
    println!("{}", s.m());// 

    let b: Box<S> = Box::new(S{i: 100});
    f(b); // 动态调度
}