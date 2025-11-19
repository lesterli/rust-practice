## Module 1: Basic syntax

By the end of this module, you should be able to:

1.  Declare variables and understand Rust's **immutability-by-default** philosophy.
2.  Use Rust's basic data types.
3.  Define and call functions, understanding how Rust uses **expressions** and **statements**.
4.  Implement basic **Control Flow** constructs like `if/else`.

### 1. Variables and Mutability

  * **Immutability:** Variables are **immutable** by default. Once a value is bound to a name using `let`, it cannot be changed.
    ```rust
    let x = 5; // Immutable
    // x = 6; // ERROR!
    ```
  * **Mutability:** Use the **`mut`** keyword to explicitly opt-in to making a variable changeable.
    ```rust
    let mut y = 5; // Mutable
    y = 6; // OK
    ```
  * **Constants:** Defined with the **`const`** keyword. They must be set at compile time, and their type **must** be annotated.
    ```rust
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    ```
  * **Shadowing:** Declaring a *new* variable with the same name as a previous one. This is distinct from mutation and allows you to change the type while keeping the same name.
    ```rust
    let spaces = "   "; // String type
    let spaces = spaces.len(); // u8 type (shadowing)
    ```

### 2. Data Types

Rust is a **statically typed** language, meaning it must know the type of all variables at compile time, though it often uses **type inference** to deduce the type for you.

  * **Scalar Types (Single Value):**

      * **Integers:** Signed (`i8`, `i16`, `i32`, `i64`, `i128`, `isize`) and Unsigned (`u8`, `u16`, `u32`, `u64`, `u128`, `usize`). `i32` is the default.
      * **Floating-Point:** `f32` (single-precision) and `f64` (double-precision, the default).
      * **Boolean:** `bool` (`true` or `false`).
      * **Character:** `char` (four bytes, represents a Unicode Scalar Value).

  * **Compound Types (Groups of Values):**

      * **Tuples:** A fixed-length, ordered list of values of potentially **different types**.
        ```rust
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let five_hundred = tup.0; // Access by index
        ```
      * **Arrays:** A fixed-length collection of values where **every element must have the same type**. Stored on the stack.
        ```rust
        let a = [1, 2, 3, 4, 5];
        let first = a[0]; // Access by index
        ```

### 3. Functions

  * **Definition:** Functions are declared using the **`fn`** keyword. The main entry point is `fn main()`.
  * **Parameters:** Type annotations **must** be declared for all function parameters.
    ```rust
    fn add_numbers(x: i32, y: i32) { ... }
    ```
  * **Statements vs. Expressions:**
      * **Statements** perform an action but do not return a value (e.g., `let x = 6;`).
      * **Expressions** evaluate to a resulting value. In Rust, almost everything is an expression, including function calls, macros, and blocks defined with curly brackets.
  * **Return Values:** The value of the **final expression** in the function body is returned implicitly. You **do not** use a semicolon on the final expression.
    ```rust
    fn five() -> i32 { // -> i32 denotes the return type
        5 // This is an expression, no semicolon
    }
    ```

### 4. Control Flow

  * **Conditional Execution (`if/else`):** The condition in an `if` expression **must** be a `bool` (Boolean). Rust does not automatically convert non-boolean types to a boolean.
      * `if` is an **expression**, meaning it can return a value (e.g., in a `let` statement), but all arms must return the same type.
  * **Looping:**
      * **`loop`:** Repeats code indefinitely until explicitly told to stop using `break`.
          * `loop` can return a value using `break value;`.
      * **`while`:** Repeats as long as a condition remains `true`.
      * **`for`:** Used for iterating over collections (or ranges), which is the safest and most idiomatic way to loop in Rust.
        ```rust
        // Iterate through a range, 1 up to (but not including) 4
        for number in 1..4 {
            println!("{number}");
        }
        ```