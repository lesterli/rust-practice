fn add_numbers(numbers: &[i32]) -> i32 {
    let a = numbers[0];
    let b = numbers[1];

    a + b
}

fn add_numbers_with_option(numbers: &[i32]) -> Option<i32> {
    let a = numbers.get(0)?; // `get` return Option<i32>
    let b = numbers.get(1)?; // ? will early return on None
    // consider dereferencing the borrow: `*b`
    a.checked_add(*b) // returns None on overflow
}

fn main() {
    let arr_normal: [i32; 3] = [1, 2, 3];
    println!("addition: {}",add_numbers(&arr_normal));
    let arr_max: [i32; 3] = [std::i32::MAX, 2, 3];
    // thread 'main' panicked at 'attempt to add with overflow'
    // println!("addition: {}",add_numbers(&arr_max));
    println!("addition: {:?}",add_numbers_with_option(&arr_max));
}