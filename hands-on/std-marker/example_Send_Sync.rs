use std::thread;
//use std::rc::Rc;
use std::sync::{Arc, Mutex};
//use std::cell::RefCell;

fn main() {
    //let mut s = Rc::new("example".to_string());
    //let s = Arc::new(RefCell::new("example".to_string()));
    let s = Arc::new(Mutex::new("example".to_string()));
    let mut v = vec![];
    for _ in 0..2 {
        //let mut s_clone = s.clone();
        let s_clone = s.clone();
        let child = thread::spawn(move || {
            let mut s_clone = s_clone.lock().unwrap();
            s_clone.push_str(" Send and Sync!");
            println!("{:?}", s_clone);
        });
        v.push(child);
    }
    for child in v {
        child.join().unwrap();
    }
    println!("{:?}", s);
}