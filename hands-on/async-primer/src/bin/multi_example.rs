use std::error::Error;
use std::thread;
use std::time::Instant;
use std::sync::Arc;

use async_primer::read_file;

fn main() -> Result<(), Box<dyn Error>> {
    // Use move closures and share start via Arc so each thread gets an owned handle 
    let start = Arc::new(Instant::now());
    println!("[{:.3}s] program started", start.elapsed().as_secs_f64());

    // spawn two threads to read files concurrently
    let s1 = Arc::clone(&start);
    let p1 = thread::spawn(move || {
        let t = Instant::now();
        let content = read_file("data/file1.txt").expect("read file1");
        println!(
            "[{:.3}s] finished file1 (took {:.3}s)",
            s1.elapsed().as_secs_f64(),
            t.elapsed().as_secs_f64()
        );
        content
    });

    let s2 = Arc::clone(&start);
    let p2 = thread::spawn(move || {
        let t = Instant::now();
        let content = read_file("data/file2.txt").expect("read file2");
        println!(
            "[{:.3}s] finished file2 (took {:.3}s)",
            s2.elapsed().as_secs_f64(),
            t.elapsed().as_secs_f64()
        );
        content
    });

    let file1 = p1.join().unwrap();
    let file2 = p2.join().unwrap();

    dbg!(&file1);
    dbg!(&file2);

    println!("[{:.3}s] program finished", start.elapsed().as_secs_f64());
    Ok(())
}