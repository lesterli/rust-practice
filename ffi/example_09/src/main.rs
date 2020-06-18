use std::os::raw::{c_char, c_float, c_int};

#[repr(C)]
#[derive(Debug)]
pub struct CStudent {
    pub num: c_int,
    pub total: c_int,
    pub name: [c_char; 20],
    pub scores: [c_float; 3],
}

// Default constructor
impl Default for CStudent {
    fn default() -> Self {
        CStudent {
            num: 0 as c_int,
            total: 0 as c_int,
            name: [0 as c_char; 20],
            scores: [0.0 as c_float; 3],            
        }
    }
}

#[link(name = "cfoo")]
extern "C" {
    fn print_data(p_stu: *mut CStudent);
    fn fill_data(p_stu: *mut CStudent);
}


fn main() {
    // Initialization of allocated memory
    let new_stu: CStudent = Default::default();
    println!("rust side print new_stu: {:?}", new_stu);
    let box_new_stu = Box::new(new_stu);
    let p_stu = Box::into_raw(box_new_stu);

    unsafe {
        fill_data(p_stu);
        print_data(p_stu);
        //Box::from_raw(p_stu);
        println!("rust side print Bob: {:?}", Box::from_raw(p_stu));
    }
}
