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

// use std::ffi::CStr;
// use std::default::Default;
// use std::str;

// mod cpuid;

// pub struct CpuInfo {
//     /// 当前CPU的物理核数
//     pub num_cores: i32,
//     /// 逻辑处理器的数量
//     pub num_logical_cpus: i32,
//     /// 逻辑处理器总数
//     pub total_logical_cpus: i32,
// }

// /// 返回 libcpuid 错误字符串
// pub fn error() -> String {
//     unsafe {
//         let ptr = cpuid::cpuid_error();
//         let bytes = CStr::from_ptr(ptr).to_bytes();
//         str::from_utf8(bytes).ok().expect("Invalid UTF8 string").to_string()
//     }
// }

// pub fn identify() -> Result<CpuInfo, String> {
//     let mut raw: cpuid::cpu_raw_data_t = Default::default();
//     let raw_result = unsafe {
//         cpuid::cpuid_get_raw_data(&mut raw)
//     };
//     if raw_result != 0 {
//         return Err(error());
//     }
//     let mut data: cpuid::cpu_id_t = Default::default();
//     let identify_result = unsafe {
//         cpuid::cpu_identify(&mut raw, &mut data)
//     };
//     if identify_result != 0 {
//         Err(error())
//     } else {
//         Ok(CpuInfo {
//             num_cores: data.num_cores,
//             num_logical_cpus: data.num_logical_cpus,
//             total_logical_cpus: data.total_logical_cpus,
//         })
//     }
// }

// fn main() {
//     match identify() {
//         Ok(info) => {
//             println!("The processor has {} cores and {} logical processors",
//                      info.num_cores,
//                      info.num_logical_cpus);
//         }
//         Err(err) => println!("cpuid error: {}", err),
//     }
// }

use std::str;

mod time;

fn main() {
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