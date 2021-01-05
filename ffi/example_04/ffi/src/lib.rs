use std::os::raw::{c_char, c_uint};
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn fibonacci(index: c_uint) -> c_uint {
    if index <= 2 {
        1
    } else {
        fibonacci(index - 1) + fibonacci(index - 2)
    }
}

#[no_mangle]
pub extern "C" fn count_char(s: *const c_char) -> c_uint {
    let c_str = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    };
    let r_str = c_str.to_str().unwrap();
    r_str.chars().count() as u32
}
