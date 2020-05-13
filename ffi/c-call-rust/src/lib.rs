use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::slice;

#[no_mangle]
pub extern "C" fn call_from_rust() {
    println!("This is a Rust function for C!");
}

// #[no_mangle]
// pub extern rust_printer(input: *const c_char) -> *mut c_char {
//     let mut hello = String::from("Hello World!");
//     let c_str_to_print = CString::new(hello).unwrap();
//     // 使用 as_ptr 将 CString 转化成 char 指针传给 C 函数
//     c_str_to_print.as_ptr()
// }


#[no_mangle]
pub extern fn sum(array: *const c_int, length: c_int) -> c_int {
    assert!(!array.is_null(), "Null pointer in sum()");
    unsafe {
        let array: &[c_int] = slice::from_raw_parts(array, length as usize);
        array.into_iter().sum()
    }
}
