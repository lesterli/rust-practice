use std::os::raw::c_int;


#[repr(C)]
pub struct OpaqueObject {
    _private: [u8; 0],
}

extern "C" {
    pub fn free_object(obj: *mut OpaqueObject);
    pub fn init() -> *mut OpaqueObject;
    pub fn get_api_version() -> c_int;
    pub fn get_info(obj: *const OpaqueObject) -> c_int;
    pub fn set_info(obj: *mut OpaqueObject, info: c_int);
}

fn main() {
    unsafe {
        let obj = init();
        println!("Original value: {}", get_info(obj));

        set_info(obj, 521);
        println!("New value: {}", get_info(obj));
    }
}