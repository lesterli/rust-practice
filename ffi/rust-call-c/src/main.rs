// use std::ffi::CString;
// use std::os::raw::c_char;

// // 外部块
// extern "C" {
//     // 标准库<stdlib.h> abs函数
//     #[link_name = "abs"]
//     fn abs_in_rust(input: i32) -> i32;

//     #[link_name = "printf"]
//     fn printf_in_rust(input: *const c_char) -> i32;
// }

// fn main() {
//     unsafe {
//         println!("abs(-1) is {}", abs_in_rust(-1));
//         let c_func_print = CString::new()
//     }
// }