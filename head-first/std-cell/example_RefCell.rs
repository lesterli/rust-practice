use std::cell::RefCell;

fn main() {
    let c = RefCell::new(5);
    *c.borrow_mut() = 7;
    assert_eq!(7, *c.borrow());

    let x = RefCell::new(vec![1,2,3]);
    println!("{:?}", x.borrow());
    x.borrow_mut().push(4);
    println!("{:?}", x.borrow());
}