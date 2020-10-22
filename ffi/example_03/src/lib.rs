use std::os::raw::{c_char, c_float, c_int};

#[repr(C)]
#[derive(Debug)]
pub struct student {
    pub num: c_int,
    pub total: c_int,
    pub name: [c_char; 20],
    pub scores: [c_float; 3],
}

// Default constructor
impl Default for student {
    fn default() -> Self {
        student {
            num: 0 as c_int,
            total: 0 as c_int,
            name: [0 as c_char; 20],
            scores: [0.0 as c_float; 3],            
        }
    }
}

#[no_mangle]
pub extern "C" fn student_new() -> *mut student {
    let new_stu: student = Default::default();
    Box::into_raw(Box::new(new_stu))
}

#[no_mangle]
pub extern "C" fn student_alice() -> *mut student {
    let mut init_char_array: [c_char; 20] = [0; 20];
    for (dest, src) in init_char_array.iter_mut().zip(b"Alice\0".iter()) {
        *dest = *src as _;
    }
    let scores = [92.5, 87.5, 90.0];
    let alice = student {
        num: 001,
        total: 280,
        name: init_char_array,
        scores,
    };
    Box::into_raw(Box::new(alice))
}

#[no_mangle]
pub extern "C" fn student_free(p_stu: *mut student) {
    if !p_stu.is_null() {
        unsafe {
            println!("rust side print: {:?}", Box::from_raw(p_stu));
            Box::from_raw(p_stu)
        };
    }
}