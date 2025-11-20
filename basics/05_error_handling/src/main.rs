// Exercises for Module 5: Error Handling and Testing
// Run: cargo run

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Error Handling & Testing ===\n");

    // Demonstrate parse_number
    match error_handling::parse_number("123") {
        Ok(n) => println!("parse_number -> {}", n),
        Err(e) => println!("parse_number error: {}", e),
    }

    // Demonstrate safe_divide
    match error_handling::safe_divide(10, 0) {
        Ok(r) => println!("10 / 0 = {}", r),
        Err(e) => println!("safe_divide error: {}", e),
    }

    // Demonstrate read_username_from_file using a temporary file
    let tmp = std::env::temp_dir().join("rust_demo_user.txt");
    std::fs::write(&tmp, "demo_user\n")?;
    let user = error_handling::read_username_from_file(tmp.to_str().unwrap())?;
    println!("read_username_from_file -> '{}'", user);
    let _ = std::fs::remove_file(tmp);

    println!("\nRun tests: cargo test");

    Ok(())
}