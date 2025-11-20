// Requires feature "async-tokio" (see Cargo.toml)
use std::error::Error;
use std::time::Instant;

#[cfg(feature = "async-tokio")]
use async_primer::read_file_async;

#[cfg(feature = "async-tokio")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    println!("[{:.3}s] async program started", start.elapsed().as_secs_f64());

    // read two files concurrently using tokio::join!
    let t1 = Instant::now();
    let f1 = tokio::spawn(async { read_file_async("data/file1.txt").await });
    let f2 = tokio::spawn(async { read_file_async("data/file2.txt").await });

    let r1 = f1.await??;
    println!(
        "[{:.3}s] processed file 1 (read took {:.3}s)",
        start.elapsed().as_secs_f64(),
        t1.elapsed().as_secs_f64()
    );

    let t2 = Instant::now();
    let r2 = f2.await??;
    println!(
        "[{:.3}s] processed file 2 (read took {:.3}s)",
        start.elapsed().as_secs_f64(),
        t2.elapsed().as_secs_f64()
    );

    dbg!(&r1);
    dbg!(&r2);

    println!("[{:.3}s] async program finished", start.elapsed().as_secs_f64());
    Ok(())
}

#[cfg(not(feature = "async-tokio"))]
fn main() {
    eprintln!("async_example requires feature \"async-tokio\". Run with:\n  cargo run --features async-tokio --bin async_example");
}