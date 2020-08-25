use std::os::raw::{c_int, c_void};

pub type SumSquareCB = unsafe extern fn(c_int, *mut c_void);

#[link(name = "sumsquare")]
extern {
    pub fn sum_square_cb(a: c_int, b: c_int, cb: SumSquareCB, user_data: *mut c_void);
}

unsafe extern fn hook<F>(result: c_int, user_data: *mut c_void)
where
    F: FnMut(c_int),
{
    (*(user_data as *mut F))(result)
}

pub fn get_callback<F>(_closure: &F) -> SumSquareCB
where
    F: FnMut(c_int),
{
    hook::<F>
}

#[derive(Debug, Default, Clone, PartialEq)]
struct SumRecord {
    total: c_int,
    calls: usize,
}

fn main() {
    let mut record = SumRecord::default();
    
    unsafe {
        let mut closure = |result: c_int| {
            record.total += result;
            record.calls += 1;
        };
        let callback = get_callback(&closure);

        sum_square_cb(1, 2, callback, &mut closure as *mut _ as *mut c_void);

        sum_square_cb(3, 4, callback, &mut closure as *mut _ as *mut c_void);
    }

    println!("The sum is {:?}", record);
}
