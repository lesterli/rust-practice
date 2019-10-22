pub fn generics_trait(){
    trait Add<RHS, Output> {
        fn my_add(self, rhs: RHS) -> Output;
    }
    impl Add<i32, i32> for i32 {
        fn my_add(self, rhs: i32) -> i32 {
            self + rhs
        }
    }
    impl Add<u32, i32> for u32 {
        fn my_add(self, rhs: u32) -> i32 {
            (self + rhs ) as i32
        }
    }

    let (a, b, c, d) = (1i32, 2i32, 3u32, 4u32);
    let x: i32 = a.my_add(b);
    let y: i32 = c.my_add(d);
    assert_eq!(x, 3i32);
    assert_eq!(y, 7i32);
}

fn main() {
    generics_trait();
}