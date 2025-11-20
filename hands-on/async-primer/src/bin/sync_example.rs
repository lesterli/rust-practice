use std::io;
use std::time::Instant;

use async_primer::read_file;

fn main() -> io::Result<()> {
    let start = Instant::now();
    println!("[{:.3}s] program started", start.elapsed().as_secs_f64());

    let t1 = Instant::now();
    let file1 = read_file("data/file1.txt")?;
    println!("[{:.3}s] processed file 1 (read took {:.3}s)", start.elapsed().as_secs_f64(), t1.elapsed().as_secs_f64());

    let t2 = Instant::now();
    let file2 = read_file("data/file2.txt")?;
    println!("[{:.3}s] processed file 2 (read took {:.3}s)", start.elapsed().as_secs_f64(), t2.elapsed().as_secs_f64());

    dbg!(&file1);
    dbg!(&file2);

    println!("[{:.3}s] program finished", start.elapsed().as_secs_f64());
    Ok(())
}