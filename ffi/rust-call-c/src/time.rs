use libc::{c_int, size_t};

#[repr(C)]
pub struct tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
}


extern {
    // 标准库<time.h> strftime函数的 Rust FFI 绑定
    #[link_name = "strftime"]
    pub fn strftime_in_rust(stra: *mut u8, maxsize: size_t, format: *const u8, timeptr: *mut tm) -> size_t;
}