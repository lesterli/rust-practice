## 🛑 Module 5: Error Handling and Testing

This module covers the final steps of writing production-ready Rust code: managing failure gracefully using Rust's distinct error handling model, and verifying correct behavior using the built-in testing features.

By the end of this module, you should be able to:

1.  Differentiate between **recoverable** and **unrecoverable** errors in Rust.
  * Use the `Result<T, E>` enum and the `?` operator for professional error propagation.
2.  Write automated **Unit Tests** and **Integration Tests** using the `#[test]` attribute.
  * Control and configure the test runner using **`cargo test`** flags.

### 1. Error Handling

Rust distinguishes between two types of errors, forcing the programmer to handle them explicitly, which leads to more robust software.

**Unrecoverable Errors** (`panic!`)

  * **Purpose**: Used when a program reaches a state that should **never** happen, indicating an irrecoverable bug (e.g., accessing an index beyond an array's bounds).
  * **Action**: The `panic!` macro starts the process of **unwinding** (cleaning up the stack and freeing memory) or **aborting** (ending the program immediately).
  * **Best Practice**: Use `panic!` only for logic errors, prototypes, or tests where failure is expected. For core application logic, use `Result`.

**Recoverable Errors** (`Result<T, E>`)

  * **Purpose**: Used when an error is expected and can be handled (e.g., file not found, network failure).
  * **The `Result` Enum**: This is a generic enum that represents either success or failure:
    ```rust
    enum Result<T, E> {
        Ok(T), // Success: Contains the successful return value of type T
        Err(E), // Failure: Contains the error value of type E
    }
    ```
  * **Handling with `match`**: The most explicit way to handle a `Result` is by using a `match` expression to take different actions based on `Ok` or `Err`.

**Error Propagation**

The `?` operator simplifies error handling by acting as shorthand for a `match` expression:

  * If the `Result` is `Ok`, the `?` operator extracts the value inside and continues execution.
  * If the `Result` is `Err`, it immediately returns the error value from the current function.

The `?` operator is the preferred way to propagate errors up the call stack.

### 2. Writing Automated Tests

Rust has built-in support for testing integrated directly into the language and managed by Cargo.

**Unit Tests**

  * **Location**: Placed in the same file as the code they are testing, typically within a `mod tests { ... }` block annotated with `#[cfg(test)]`.
  * **Syntax**: Every test function is simply a regular function annotated with `#[test]`.
  * **Verification**: You use **assertion macros** to check conditions:
      * `assert!(condition)`: Panics if the condition is false.
      * `assert_eq!(left, right)`: Panics if the two values are not equal.
      * `assert_ne!(left, right)`: Panics if the two values are equal.
  * **Testing `panic!`**: You can check that code *correctly* panics using the `#[should_panic]` annotation.

**Integration Tests**

  * **Purpose**: Test the public interface of your library crate as a whole, ensuring different parts work together correctly.
  * **Location**: Placed in a dedicated **`tests` directory** next to the `src` directory (e.g., `project_root/tests/`).
  * **Execution**: Each file in the `tests` directory is compiled as its own external crate and run against your library.

**Running Tests with Cargo**

The **`cargo test`** command automatically discovers and runs all tests in your project (both unit and integration tests).

| Command | Action |
| :--- | :--- |
| `cargo test` | Runs all tests in the project. |
| `cargo test test_name` | Runs only tests whose name contains `test_name`. |
| `cargo test -- --test-threads=1` | Runs tests sequentially (useful when tests require exclusive access to shared resources). |
| `cargo test -- --show-output` | Shows the output of successful tests (which is usually suppressed). |

### How to run

Run exercises:

```bash
cd basics/05_error_handling
cargo run
```

Run tests:

```bash
cd basics/05_error_handling
cargo test
```