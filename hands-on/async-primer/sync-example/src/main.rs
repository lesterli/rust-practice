use std::io;

mod file;

fn main() -> io::Result<()> {
    println!("program started");

    let file1 = file::read_file("src/file1.txt")?;
    println!("processed file 1");

    let file2 = file::read_file("src/file2.txt")?;
    println!("processed file 2");

    dbg!(&file1);
    dbg!(&file2);

    Ok(())
}