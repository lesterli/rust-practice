// 标准库<stdlib.h>内置的abs函数
extern "C" {
    #[link_name = "abs"]
    fn abs_in_rust(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("abs(-1) is {}", abs_in_rust(-1));
    }
}
