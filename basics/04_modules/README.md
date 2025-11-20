## Module 4: Project Structure and Common Collections

This module focuses on writing code that scales. You'll learn how to organize large projects using the **Module System** and how to use Rust's standard **common collections** for managing dynamic sets of data.

By the end of this module, you should be able to:

1. Differentiate between **Packages**, **Crates**, and **Modules**.
  * Control the **privacy** of code using the `pub` keyword.
  * Manage code organization and paths using `mod`, `use`, and the **module tree**.
2. Use the three standard library collections: **Vectors**, **Strings**, and **Hash Maps**.

### 1. Module System

Rust's **module system** is a hierarchy that structures and manages code within a project. It helps control scope, which items are public, and where to find the definitions of code.

**Packages and Crates**

| Term | Definition |
| :--- | :--- |
| **Package** | A feature of **Cargo** that holds one or more *crates*. It contains the `Cargo.toml` file. |
| **Crate** | The unit of compilation in Rust. A package must contain at least one crate. |
| **Binary Crate** | An executable program (e.g., `src/main.rs`). |
| **Library Crate** | Code intended to be used by other projects (e.g., `src/lib.rs`). |

**Modules and Paths**

* **Modules (`mod`)**: Modules are containers that hold definitions (functions, structs, other modules). They form a **tree structure** within a crate, starting from the crate root (`src/main.rs` or `src/lib.rs`).
* **Privacy Rules**: All items (functions, methods, etc.) are **private by default**. To make an item accessible from its parent module or outside, you must prefix it with `pub`.
* **Paths**: To refer to an item within the module tree, you use a **path**.
    * **Absolute Path**: Starts from the crate root (`crate::`).
    * **Relative Path**: Starts from the current module (`super::` to go up one level, or the item's name for a sibling module).
* **Bringing Paths into Scope (`use`)**: The `use` keyword creates a shortcut to an item, allowing you to refer to it directly without typing out the full path every time. This is standard practice for external dependencies and heavily used local modules.

### 2. Common Collections

Collections are data structures that can hold multiple values, where the amount of data is not known at compile time. Therefore, the data they manage is always stored on the **Heap**.

**Vectors** (`Vec<T>`)

* **Purpose**: Stores a variable-sized list of values of the **same type** (`T`).
* **Characteristics**: Elements are stored contiguously in memory, making access fast. Can grow or shrink at runtime.
* **Ownership**: When the `Vec` itself goes out of scope, it is dropped, and all the elements it owns are also dropped (memory is freed).

**Strings** (`String` vs. `&str`)

Rust has two primary string types:

* **`String`**: The **owner** of the data. It is a growable, mutable, **UTF-8 encoded** sequence of characters stored on the **Heap**. This is what you use when you need to modify or own text data.
* **`&str` (String Slice)**: An **immutable reference** to a portion of a `String` or a string literal (which is stored in the binary). This is what you use for fast, read-only string views.
* **Complexity**: Unlike many languages, Rust's `String` is complex due to its commitment to UTF-8 safety, requiring care when indexing or manipulating characters.

**Hash Maps** (`HashMap<K, V>`)

* **Purpose**: Stores mappings from keys (`K`) to values (`V`).
* **Characteristics**: Keys must be of the same type, and values must be of the same type. Allows fast retrieval of a value based on its key.
* **Ownership**: The Hash Map takes **ownership** of the data for both the keys and the values when they are inserted.

| Collection Type | Access Method | Ownership Rule |
| :--- | :--- | :--- |
| `Vec<T>` | Indexing (`[]`) or `.get()` | Stores owned values. |
| `String` | Methods (e.g., `.push_str()`) | Is the owner of the data. |
| `HashMap<K, V>` | `.get()` or `.insert()` | Takes ownership of keys and values. |