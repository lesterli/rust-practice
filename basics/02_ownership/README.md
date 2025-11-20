## 🛡️ Module 2: Ownership and Borrowing

This module explores **ownership** and **borrowing** that Rust's most unique and powerful features and explains how these rules enable Rust to manage memory safely.

By the end of this module, you should be able to:

1.  Understand Rust's **ownership** and how it works.
2.  Use **references** (borrowing) to share data without transferring ownership.

### 1. Ownership

Ownership is the core discipline of heap/pointer management. Three Ownership Rules:

* Each value in Rust has an owner (the variable it's assigned to).
* There can only be one owner at a time.
* When the owner goes out of scope, the value is dropped (memory is freed).

The key differences between languages are who manages the memory in the Heap. Rust introduces a third approach between garbage collection (GC) and manual management: Rust compiler (via the borrow checker) inserts the freeing logic (the call to drop) automatically at compile time when a variable goes out of scope.
* When a variable with data on the Heap (like a `String`) is assigned to another variable or passed to a function, the **ownership is transferred** (moved) and the original variable is invalidated.

```rust
let s1 = String::from("hello");
let s2 = s1; // s1's ownership is moved to s2. s1 is now invalid.
// println!("{}", s1); // Compile-time ERROR! (Use after Move)
```

* When a type with data stored on the stack (like `i32` or `bool`) is assigned to a new variable, the value is simply copied, the original variable remains valid.

```rust
let x = 5; // i32 implements Copy
let y = x; // The value 5 is copied.
println!("x is {}, y is {}", x, y); // Both are valid
```

### 2. References and Borrowing

References provide the ability to read and write data without consuming ownership of it. References are created with borrows (immutable references `&` and mutable references `&mut`) and used with dereferences (`*`), often implicitly.

Rust’s borrow checker enforces a system of permissions that ensures references are used safely:

* All variables can read, own, and (optionally) write their data.
* Creating a reference will transfer permissions from the borrowed place to the reference.
* Permissions are returned once the reference’s lifetime has ended.
* Data must outlive all references that point to it.

```rust
let mut s = String::from("data");
let r1 = &s;      // OK: First immutable reference (Reader)
let r2 = &s;      // OK: Second immutable reference (Reader)
// let r3 = &mut s; // ERROR: Cannot get a mutable reference while immutable ones exist!
```