#[no_mangle]
pub extern "C" fn call_from_rust() {
    println!("This is a Rust function for C!");
}
