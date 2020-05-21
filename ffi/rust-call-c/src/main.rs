use std::ffi::CString;
use std::os::raw::c_char;

// 外部块
extern "C" {
    // 标准库<stdlib.h> abs函数
    #[link_name = "abs"]
    fn abs_in_rust(input: i32) -> i32;

    #[link_name = "printf"]
    fn printf_in_rust(input: *const c_char) -> i32;
}

fn abs_example() {
    unsafe {
        println!("abs(-1) is {}", abs_in_rust(-1));
    }
}

use std::str;

mod time;

fn time_example() {
    let mut v: Vec<u8> = vec![0; 80];
    let mut t = time::tm {
        tm_sec: 15,
        tm_min: 09,
        tm_hour: 18,
        tm_mday: 14,
        tm_mon: 04,
        tm_year: 120,
        tm_wday: 4,
        tm_yday: 135,
        tm_isdst: 0,
    };
    let format = b"%Y-%m-%d %H:%M:%S\0".as_ptr();
    unsafe {
        time::strftime_in_rust(v.as_mut_ptr(), 80, format, &mut t);

        let s = match str::from_utf8(v.as_slice()) {
            Ok(r) => r,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
    
        println!("result: {}", s);
    }
}

fn main() {
    abs_example();
    time_example();
}