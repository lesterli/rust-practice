// Exercises for Module 2 — Ownership and Borrowing
// Run: cargo run

fn main() {
    println!("=== Ownership & Borrowing exercises ===\n");

    ownership_move_and_clone();
    copy_types_demo();
    borrowing_rules_demo();
    mutable_borrow_example();
    fix_ownership_error_example();
    slice_examples();

    println!("\n=== End ===");
}

/// Demonstrate move semantics and how to keep data with clone().
fn ownership_move_and_clone() {
    println!("-- ownership_move_and_clone --");

    let s1 = String::from("hello");
    // Move: s2 takes ownership of the heap data
    let s2 = s1;
    println!("s2 = {}", s2);
    // s1 is now invalid (use of s1 here would be a compile error).
    //
    // If you need to keep using the original, clone the data:
    let s3 = String::from("world");
    let s4 = s3.clone(); // deep copy of heap data
    println!("s3 = {}, s4 = {}", s3, s4); // both usable
}

/// Types that implement Copy are copied, not moved.
fn copy_types_demo() {
    println!("-- copy_types_demo --");

    let x = 42; // i32 implements Copy
    let y = x;  // copy, x still valid
    println!("x = {}, y = {}", x, y);

    let b = true;
    let c = b;
    println!("b = {}, c = {}", b, c);
}

/// Immutable borrowing: multiple readers allowed.
fn borrowing_rules_demo() {
    println!("-- borrowing_rules_demo --");

    let s = String::from("read-only");
    let r1 = &s;
    let r2 = &s;
    // multiple immutable borrows are fine
    println!("r1 = {}, r2 = {}", r1, r2);

    // declare s2 outside the inner block so we can mutate it later
    let mut s2 = String::from("changeable");
    {
        let r3 = &s2; // immutable borrow
        println!("r3 = {}", r3);
        // r3's last use is above, so the mutable borrow below is allowed
    } // r3 goes out of scope here

    // now take a mutable borrow and mutate s2
    {
        let mr = &mut s2;
        mr.push_str("!");
        println!("mr after mutation = {}", mr);
    }

    println!("s2 final = {}", s2);
}

/// Mutable borrowing: only one mutable reference at a time.
fn mutable_borrow_example() {
    println!("-- mutable_borrow_example --");

    let mut s = String::from("hello");
    {
        let mr = &mut s; // one mutable borrow
        mr.push_str(" world");
        println!("mr = {}", mr);
    } // mr goes out of scope here, returning borrow to owner

    // Now we can borrow mutably again or immutably
    let r = &s;
    println!("after mutation, r = {}", r);
}

/// Example showing how to fix common ownership errors by borrowing.
fn fix_ownership_error_example() {
    println!("-- fix_ownership_error_example --");

    let mut s = String::from("fix me");
    append_exclamation(&mut s); // pass a mutable reference
    println!("s after append_exclamation = {}", s);

    // BAD pattern (for reference): returning a reference to a local value is disallowed.
    // The following would not compile if uncommented:
    //
    // fn bad_return() -> &String {
    //     let s = String::from("temp");
    //     &s // ERROR: cannot return reference to local variable `s`
    // }
    //
    // The fix: return the owned String instead, or accept an input reference and return a slice.
}

/// Appends an exclamation mark via a mutable borrow.
fn append_exclamation(s: &mut String) {
    s.push('!');
}

/// Slice examples: &str and &[T]
fn slice_examples() {
    println!("-- slice_examples --");

    // string slice (&str)
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("first_word of '{}' is '{}'", s, word);

    // &str can be taken from a string literal too
    let lit: &str = "static slice";
    println!("literal slice = {}", lit);

    // array slice (&[T])
    let arr = [10, 20, 30, 40];
    let mid = &arr[1..3]; // &[i32]
    println!("arr = {:?}, mid slice = {:?}", arr, mid);

    // slice a String as bytes and convert to &str where valid UTF-8
    let s2 = String::from("rustacean");
    let part = &s2[0..4]; // "rust"
    println!("part of '{}': {}", s2, part);
}

/// Return the first word as a &str slice — idiomatic safe slice example.
fn first_word(s: &str) -> &str {
    // iterate bytes to find the first space
    for (i, &b) in s.as_bytes().iter().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}