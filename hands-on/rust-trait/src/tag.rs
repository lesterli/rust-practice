pub fn test_copy_trait(){
    fn test_copy<T: Copy>(i: T) {
        println!("hhh");
    }
    let a = "String";
    test_copy(a);
}

pub fn sync_send_trait(){
    use std::thread;
    let mut x = vec![1, 2, 3, 4];
    thread::spawn(move || x.push(1));
}

fn main() {
    test_copy_trait();
    sync_send_trait();
}