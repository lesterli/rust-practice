use std::os::raw::{c_char, c_float, c_int};

#[repr(C)]
#[derive(Debug)]
pub struct Student {
    pub num: c_int,
    pub total: c_int,
    pub name: [c_char; 20],
    pub scores: [c_float; 3],
}

// Default constructor
impl Default for Student {
    fn default() -> Self {
        Student {
            num: 0 as c_int,
            total: 0 as c_int,
            name: [0 as c_char; 20],
            scores: [0.0 as c_float; 3],            
        }
    }
}

#[no_mangle]
pub extern "C" fn student_new() -> *mut Student {
    let new_stu: Student = Default::default();
    Box::into_raw(Box::new(new_stu))
}

#[no_mangle]
pub extern "C" fn student_alice() -> *mut Student {
    let mut init_char_array: [c_char; 20] = [0; 20];
    for (dest, src) in init_char_array.iter_mut().zip(b"Alice\0".iter()) {
        *dest = *src as _;
    }
    let scores = [92.5, 87.5, 90.0];
    let alice = Student {
        num: 001,
        total: 280,
        name: init_char_array,
        scores,
    };
    Box::into_raw(Box::new(alice))
}

#[no_mangle]
pub extern "C" fn student_free(p_stu: *mut Student) {
    if !p_stu.is_null() {
        unsafe {
            println!("rust side print: {:?}", Box::from_raw(p_stu));
            Box::from_raw(p_stu)
        };
    }
}