fn main() {
    let mut upp = String::with_capacity(512);
    for arg in std::env::args().skip(1) {
        upp.clear();
        uppercase(&arg, &mut upp);
        println!("upp = {}", upp);
        println!("arg = {}", arg);
    }
}

// was `mut dst: String`
fn uppercase(src: &str, dst: &mut String) {
    for c in src.chars() {
        for c in c.to_uppercase() {
            dst.push(c);
        }
    }
}