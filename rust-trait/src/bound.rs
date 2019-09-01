pub fn trait_bound(){
    use std::ops::Add;
    fn sum<T: Add<T, Output=T>>(a: T, b: T) -> T{
        a + b
    }
    assert_eq!(sum(1u32, 2u32), 3);
    assert_eq!(sum(1u64, 2u64), 3);
}

fn main() {
    trait_bound();
}