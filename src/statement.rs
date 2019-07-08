fn main() {
    pub fn answer() -> () {
        // 声明语句
        let a = 40;
        let b = 2;
        // 表达式语句：以分号结尾的表达式
        // println! 宏语句：名字以叹号结尾，并像函数一样被调用
        println!("40 + 2 = {}", sum(a, b));
    }
    /// 求和函数
    pub fn sum(a: i32, b: i32) -> i32 {
        // 表达式，无分号返回求值结果
        a + b
        // Rust中不用return关键字，从代码可读性是否加上好些？
        // return a + b;
    }
    answer();
}