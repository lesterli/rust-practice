use std::os::raw::{c_char, c_uint};
use std::ffi::CStr;
use std::convert::From;

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

// A struct that can be passed between C and Rust
#[repr(C)]
pub struct c_tuple {
    integer: c_uint,
    boolean: bool,
}

impl From<c_tuple> for (u32, bool) {
    fn from(tup: c_tuple) -> (u32, bool) {
        (tup.integer, tup.boolean)
    }
}

impl From<(u32, bool)> for c_tuple {
    fn from(tup: (u32, bool)) -> c_tuple {
        c_tuple {
            integer: tup.0,
            boolean: tup.1,
        }
    }
}

#[no_mangle]
pub extern "C" fn handle_tuple(tup: c_tuple) -> c_tuple {
    let (integer, boolean) = tup.into();

    (integer + 1, !boolean).into()
}
