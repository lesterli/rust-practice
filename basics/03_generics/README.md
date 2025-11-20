## ⚙️ Module 3: Generics and Traits

This module focuses on **code design** and **reusability**. **Generics** and **Traits** are the primary tools Rust uses to write flexible code that avoids duplication while maintaining performance and static safety.

By the end of this module, you should be able to:

1.  Use **Generics** (`<T>`) to define functions, structs, and enums that work with any data type.
2.  Define **Traits** to declare shared behavior (methods) that different types can implement.
    * Combine Generics and Traits using **Trait Bounds** to specify constraints on generic types.
    * Use common **Derivable Traits** (e.g., `Debug`, `Clone`).

### 1. Generics: abstraction over types

**Generics** are abstract placeholders for concrete types or other properties. They allow you to define code once, and have it work correctly for multiple different types, eliminating code duplication.

  * **Syntax:** Generic type parameters are declared using angle brackets (`<T>`, `<K, V>`, etc.) after the item name.
  * **Generic Functions:** Used when the logic is the same regardless of the input type.
    ```rust
    fn largest<T>(list: &[T]) -> &T { /* ... */ }
    ```
  * **Generic Structs and Enums:** Used for container types that can hold different kinds of data. **Standard library types** like `Option<T>`, `Result<T, E>`, and `Vec<T>` are all defined using generics.
    ```rust
    struct Point<T> { x: T, y: T }
    ```
  * **Performance:** Rust handles generics through **monomorphization**. At compile time, the compiler effectively creates specific, optimized copies of the generic code for every concrete type used (e.g., one function for `largest<i32>` and another for `largest<char>`). This guarantees the performance of static dispatch with zero runtime cost.

### 2. Traits: defining shared behavior

A **Trait** defines a contract for behavior, a set of methods that a type must implement. They are similar to **interfaces** in other languages and are the primary mechanism for polymorphism in Rust.

  * **Definition:** You define a trait using the `trait` keyword.
    ```rust
    pub trait Summary {
        fn summarize(&self) -> String;
        // Traits can also contain default implementations
        fn read_more_link(&self) -> String {
            String::from("(Read more...)")
        }
    }
    ```
  * **Implementation:** You implement a trait for a specific type using the `impl` block. A type can implement multiple traits.
    ```rust
    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("{} by {}", self.title, self.author)
        }
    }
    ```
  * **Derivable Traits:** Many common traits (`Debug`, `Clone`, `Copy`, `PartialEq`) can be automatically implemented by the compiler using the `#[derive]` macro.

  * **Trait Bounds** allow you to restrict a generic type parameter `T` to only types that implement a specific behavior (trait).
    | Syntax | Example | Description |
    | :--- | :--- | :--- |
    | **`impl` Trait** | `fn notify(item: &impl Summary)` | Syntactic sugar for a single trait bound. |
    | **Colon Notation** | `fn notify<T: Summary>(item: &T)` | The classic way to constrain a generic type `T`. |
    | **Multiple Bounds** | `fn notify<T: Summary + Display>(item: &T)` | Requires the type `T` to implement *both* traits. |
    | **`where` Clause** | `fn notify<T>(item: &T) where T: Summary + Display` | Used to clean up function signatures when dealing with many bounds. |