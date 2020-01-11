## How to understand Rust's default thread safety?

This article uses `Rc` and `RefCell` as examples to discuss how `Send` and `Sync` in Rust ensure thread safety.

### Basic concepts

`Send` and `Sync` are located in the standard library `std::marker` module. They are "marker traits", meanint they have no methods and don't inherently provide any functionality. Specifically, the working definitions we have from them now are::

* If type `T` implements `Send`, then passing by-value a value of type `T` into another thread will not lead to data rases;
* If type `T` implements `Sync`, then passing a reference `&T` to a value of type `T` into another thread will not lead to data races (aka, `T: Sync` implies `&T: Send`)

That is, `Sync` is related to how a type works when shared across multiple threads at once, and `Send` talks about how a type behaves as it crosses a task boundary.

> Allowing Transference of Ownership Between Threads with Send Send

> Allowing Access from Multiple Threads with Sync

In the `std::marker` module of Rust's standard library, `Send` and `Sync` are implemented by default for all types.

### Thread

Rust's thread-related content is in the standard library `std::thread` module. Threads in Rust are a direct encapsulation of operating system threads. That is to say, it is a local thread, and each thread has its own stack and local state. Example:

```Rust
use std::thread;
use std::time::Duration;

fn main() {
	 // Specify some parameters for the thread t using Builder mode
    let t = thread::Builder::new()
        .name("t".to_string())
        // Create thread t and use move to transfer ownership
        .spawn(move || {
        	  // Execution logic of thread t
            println!("enter child thread.");
            // Pause the current thread and enter the wait state
            thread::park();
            println!("resume child thread");
        }).unwrap();
    println!("spawn a thread");
    // Make the current thread wait for a period of time to continue execution
    thread::sleep(Duration::new(5,0));
    // Resume execution of thread t
    t.thread().unpark();
    // The main thread waits for thread t to end
    t.join();
    println!("child thread finished");
}
```

The function signature of the `std::thread::spawn` function is as follows:

```Rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T> 
where F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
```

From the function signature we can know that `spawn()` accepts a callable (usually a closure), which is called once and contains the data of `Send` and `'static`. That is, only types that implement `Send` can be passed between threads.

At the same time, the `'static` restriction prevents borrowed data from being shared between threads. Closures can capture external variables, but by default it is captured by reference. Without the `move` keyword in the example code, the closure will not be `'static`, because it contains borrowed data.

### Examples: `Rc` and `RefCell`
 
**Question**: How to pass mutable strings between threads?

`Rc` stands for "Reference Counted", a single-threaded reference count pointer. Example code:

```Rust
use std::thread;
use std::rc::Rc;

fn main() {
    let mut s = Rc::new("example".to_string());
    for _ in 0..2 {
        let mut s_clone = s.clone();
        thread::spawn(move || {
            s_clone.push_str(" Send and Sync!");
        });
    }
}
```

The Rust compiler will report an error. The error message tells us that `std::rc::Rc<std::string::String>` cannot be safely sent between threads.

This is because `Rc<String>` does not implement `Send`. We could try to use `Arc<T>` to share ownership. Example code:

```Rust
use std::thread;
//use std::rc::Rc;
use std::sync::Arc;

fn main() {
    //let mut s = Rc::new("example".to_string());
    let s = Arc::new("example".to_string());
    for _ in 0..2 {
        //let mut s_clone = s.clone();
        let s_clone = s.clone();
        thread::spawn(move || {
            s_clone.push_str(" Send and Sync!");
        });
    }
}
```

The compiler still gives an error. The error message tells us that immutable borrowing is treated as variable borrowing.

This is because `Arc<T>` is immutable by default. We can use the types with internal variability mentioned in the previous article.

`RefCell` represents a variable memory location, and the borrow rules are checked at runtime. Example code:

```Rust
use std::thread;
//use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;

fn main() {
    //let mut s = Rc::new("example".to_string());
    let s = Arc::new(RefCell::new("example".to_string()));
    for _ in 0..2 {
        //let mut s_clone = s.clone();
        let s_clone = s.clone();
        thread::spawn(move || {
            let s_clone = s_clone.borrow_mut();
            s_clone.push_str(" Send and Sync!");
        });
    }
}
```

The compiler gives an error again, and the error message tells us that `std::cell::RefCell<std::string::String>` cannot be safely shared between threads.

This is because `RefCell<String>` does not implement `Sync`.

### Conclusion

For thread safety, Rust has a different design than other languages. Rust uses the two marker traits `Send` and `Sync` to “tag” the type, and the compiler recognizes whether the type can be moved or shared between multiple threads, finds problems during compile time, eliminates data races, and ensures thread safety.

The code in the Example section of this article compile with error but click "[this](https://github.com/lesterli/rust-practice/blob/master/head-first/std-marker/example_Send_Sync.rs)" to get the code that works.