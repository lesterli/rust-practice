fn main() {
    let arg = std::env::args()
        .skip(1)
        .next()
        .expect("should have one argument");

    println!("upp = {}", uppercase(arg.clone()));
    println!("arg = {}", arg);
}

fn uppercase(s: String) -> String {
    s.to_uppercase()
}