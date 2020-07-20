use std::os::raw::c_char;
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn generate(s: *const c_char) -> *mut c_char {
    let c_str = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    };

    let r_str = c_str.to_str().unwrap();
    println!("rust side: {:?}", r_str);
    let mut ping = String::from(r_str);
    if r_str == "ping" {
        ping.push_str(" - pong");
    }
    let c_str_song = CString::new(ping).unwrap();
    c_str_song.into_raw()
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
