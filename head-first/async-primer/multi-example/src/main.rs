use std::io;
use std::sync::RwLock;
use std::thread;

use lazy_static::lazy_static;

mod file;

// A sync primitive that allows to read/write from variables between threads.
// we declare the variables here, this requires the lazy_static crate
lazy_static! {
    static ref FILE1: RwLock<String> = RwLock::new(String::from(""));
    static ref FILE2: RwLock<String> = RwLock::new(String::from(""));
}

fn main() -> io::Result<()> {
    println!("program started");

    let thread_1 = thread::spawn(|| {
        let mut w1 = FILE1.write().unwrap();
        *w1 = file::read_file("src/file1.txt").unwrap();
        println!("read file 1");
    });

    println!("Launched Thread 1");

    let thread_2 = thread::spawn(|| {
        let mut w2 = FILE2.write().unwrap();
        *w2 = file::read_file("src/file2.txt").unwrap();
        println!("read file 2");
    });

    println!("Launched Thread 2");

    let mut rf1: bool = false;
    let mut rf2: bool = false;

    loop {
        let r1 = FILE1.read().unwrap();
        let r2 = FILE2.read().unwrap();

        if *r1 != String::from("") && rf1 == false {
            println!("completed file 1");
            rf1 = true;
        }

        if *r2 != String::from("") && rf2 == false {
            println!("completed file 2");
            rf2 = true;
        }
    }

    Ok(())
}