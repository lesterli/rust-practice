use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn call_from_rust() {
    println!("This is a Rust function for C!");
}

#[no_mangle]
pub extern rust_printer(input: *const c_char) -> *mut c_char {
    let mut hello = String::from("Hello World!");
    let c_str_to_print = CString::new(hello).unwrap();
    // 使用 as_ptr 将 CString 转化成 char 指针传给 C 函数
    c_str_to_print.as_ptr()
}
