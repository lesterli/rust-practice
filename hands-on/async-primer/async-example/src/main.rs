use futures::executor::block_on;
use futures::join;
use std::io;

mod file;

// fn main() -> io::Result<()> {
//     let r1 = file::read_file("src/file1.txt");
//     let r2 = file::read_file("src/file2.txt");

//     let f1 = r1.await;
//     let f2 = r2.await;

//     dbg!(f1);
//     dbg!(f2);

//     Ok(())
// }


fn main() -> io::Result<()> {

    println!("Program started");

    // Block on the final future
    block_on(load_files());

    Ok(())
}

async fn load_files() {
    // Join the two futures together
    join!(load_file_1(), load_file_2());
}

async fn load_file_1() {
    let r1 = file::read_file("src/file1.txt").await;
    println!("file 1 size: {}", r1.unwrap().len());
}

async fn load_file_2() {
    let r2 = file::read_file("src/file2.txt").await;
    println!("file 2 size: {}", r2.unwrap().len());
}