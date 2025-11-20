use std::ops::Deref;  

#[derive(Debug)]   
struct MyBox<T> {
    value : T,  
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.value
    } 
}

fn main() {
    let instance = MyBox{value : 10};
    assert_eq!(10, *instance);
    println!("{}, {}", *instance, *(instance.deref()));
}