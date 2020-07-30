use std::os::raw::{c_char, c_float, c_void};
use std::ffi::CStr;
use std::panic::catch_unwind;

fn may_panic() {
    if rand::random() {
        panic!("panic happens");
    }
}

#[no_mangle]
pub unsafe extern "C" fn no_panic() -> i32 {
    let result = catch_unwind(may_panic);
    match result {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[derive(Debug)]
enum Version { Version1, Version2 }

fn parse_version(header: &str) -> Result<Version, &'static str> {
    match header {
        "" => Err("invalid header length"),
        "v1" => Ok(Version::Version1),
        "v2" => Ok(Version::Version2),
        _ => Err("invalid version"),
    }
}

#[no_mangle]
pub unsafe extern "C" fn handle_result(s: *const c_char) -> i32 {
    if (s as *mut c_void).is_null() {
        return -1;
    }

    let vb = CStr::from_ptr(s).to_str().unwrap();
    let version = parse_version(vb);
    match version {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

fn divide(numerator: f32, denominator: f32) -> Option<f32> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

#[no_mangle]
pub unsafe extern "C" fn handle_option(x: c_float, y: c_float) -> i32 {
    // The return value of the function is an option
    let result = divide(x, y);

    // Pattern match to retrieve the value
    match result {
        // The division was valid
        Some(_) => 0,
        // The division was invalid
        None    => -1,
    }
}

