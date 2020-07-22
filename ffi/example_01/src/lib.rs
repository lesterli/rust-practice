use std::os::raw::c_char;
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn print_str(s: *const c_char) {
    let slice = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    };
    let r_str = slice.to_str().unwrap();
    println!("Rust side print: {:?}", r_str);
}

#[no_mangle]
pub extern "C" fn change_str(s: *mut c_char) -> *mut c_char {
    let mut string = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s).to_string_lossy().into_owned()
    };
    string.push_str(" World!");
    println!("Rust side change: {:?}", string);
    let c_str_changed = CString::new(string).unwrap();
    c_str_changed.into_raw()
}

#[no_mangle]
pub extern "C" fn generate_str() -> *mut c_char {
    let ping = String::from("ping");
    println!("Rust side generate: {:?}", ping);
    let c_str_ping = CString::new(ping).unwrap();
    c_str_ping.into_raw()
}

#[no_mangle]
pub extern "C" fn free_str(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}
