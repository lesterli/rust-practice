// Exercises for Module 3 — Generics and Traits
// Run: cargo run

use std::fmt::Display;

fn main() {
    println!("=== Generics, Traits, Lifetimes ===\n");

    generics_examples();
    traits_examples();
    lifetimes_examples();

    println!("\n=== End ===");
}

/* -------------------------
   Generics examples
   ------------------------- */

fn generics_examples() {
    println!("-- generics_examples --");

    // generic function returning a reference to the largest element
    let nums = vec![3, 7, 2, 9, 4];
    let max = largest(&nums);
    println!("largest in {:?} is {}", nums, max);

    let words = vec!["apple", "pear", "banana"];
    let max_word = largest(&words);
    println!("largest in {:?} is {}", words, max_word);

    // generic struct
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "left", y: 'L' };
    println!("p1: ({}, {}), p2: ({}, {})", p1.x, p1.y, p2.x, p2.y);

    let p3 = p1.mixup(p2);
    println!("mixup -> ({}, {})", p3.x, p3.y);
}

/// return reference to largest element; T needs PartialOrd
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    assert!(!list.is_empty(), "list must be non-empty");
    let mut largest = &list[0];
    for item in &list[1..] {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    // produce a Point mixing types from two points
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

/* -------------------------
   Traits examples
   ------------------------- */

fn traits_examples() {
    println!("\n-- traits_examples --");

    let article = Article {
        headline: String::from("Generics in Rust"),
        author: String::from("Jane"),
        content: String::from("Short intro..."),
    };
    let tweet = Tweet {
        username: String::from("rustacean"),
        content: String::from("I ❤️ Rust"),
    };

    // simple use of an impl Trait argument
    notify(&article);
    notify(&tweet);

    // generic function with trait bounds and where clause
    let announced = announce_and_return_summary(&tweet, "breaking");
    println!("announce_and_return_summary -> {}", announced);
}

pub trait Summary {
    fn summarize(&self) -> String;

    // default method
    fn read_more_link(&self) -> String {
        String::from("(read more)")
    }
}

struct Article {
    headline: String,
    author: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {} with {}", self.headline, self.author, self.content)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

// syntactic sugar: impl Trait
fn notify(item: &impl Summary) {
    println!("notify: {}", item.summarize());
}

// generic version with trait bounds and where
fn announce_and_return_summary<T, U>(item: &T, ann: U) -> String
where
    T: Summary + Display,
    U: AsRef<str> + Display,
{
    println!("announcement: {}", ann);
    format!("{} -- {}", item, item.read_more_link())
}

impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tweet by {}: {}", self.username, self.content)
    }
}

/* -------------------------
   Lifetimes examples
   ------------------------- */

fn lifetimes_examples() {
    println!("\n-- lifetimes_examples --");

    let string1 = String::from("long string is long");
    let string2 = "short";

    let result = longest(string1.as_str(), string2);
    println!("longest('{}','{}') -> '{}'", string1, string2, result);

    // struct holding a reference requires an explicit lifetime
    let novel = String::from("Call me Ishmael.");
    // if no '.' is found, fall back to the entire `novel`
    let first_sentence = novel.split('.').next().unwrap_or(&novel[..]);
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("ImportantExcerpt.part = '{}'", i.part);

    // static lifetime example
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);

    // function combining lifetimes and generics
    let ann = String::from("note");
    let res = longest_with_announcement(first_sentence, string2, &ann);
    println!("longest_with_announcement -> '{}'", res);
}

// returns the longer of two string slices; the returned lifetime is tied to the inputs
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: &T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    longest(x, y)
}