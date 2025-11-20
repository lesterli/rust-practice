// Exercises for Module 4 — Modules & Collections
// Run: cargo run
mod vectors;
mod maps;
mod text;
mod slices;

fn main() {
    println!("=== Modules & Collections ===\n");

    vectors::run();
    maps::run();
    text::run();
    slices::run();

    println!("\n=== End ===");
}