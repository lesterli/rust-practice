/// 函数作为参数
pub fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    op(a,b)
}
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
fn product(a: i32, b: i32) -> i32 {
    a * b
}

/// 函数作为返回值
fn is_true() -> bool { true }
pub fn true_maker() -> fn() -> bool { is_true }

/// CTFE编译时函数执行
const fn init_len() -> usize { return 5; }

/// 匿名函数闭包作为参数
fn closure_math<F: Fn() -> i32>(op: F) -> i32 {
    // 通过添加一对圆括号，调用传入的闭包
    op()
}

/// 匿名函数闭包作为返回值
fn two_times_impl() -> impl Fn(i32) -> i32 {
    let i = 2;
    // 使用 move 转移变量 i 的所有权，避免悬挂指针，安全返回闭包
    move |j| j * i
}

/// geektime: function
fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

fn square(value: i32) -> i32 {
    value * value
}

fn cube(value: i32) -> i32 {
    value * value * value
}

fn pi() -> f64 {
    3.1415925
}

fn not_pi() {
    // 如果最后一个表达式后添加了; 分号，隐含其返回值为 unit
    3.1425926;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn_return() {
        assert_eq!(pi(), 3.1415925);
        assert_eq!(not_pi(), ());
    }

    #[test]
    fn test_apply() {
        assert_eq!(apply(2, square), 4);
        assert_eq!(apply(2, cube), 8);
    }

    #[test]
    fn test_math() {
        assert_eq!(math(sum, 2, 3), 5);
        assert_eq!(math(product, 2, 3), 6);
    }
}

fn main() {
    println!("is_pi: {:?}, is_unit1: {:?}", pi(), not_pi());

    println!("apply square: {}", apply(2, square));
    println!("apply cube: {}", apply(2, cube));

    // 默认函数名是函数类型，参数显式指定了函数的类型，被转换成函数指针类型
    let a = 2;
    let b = 3;
    println!("2+3={}", math(sum, a, b));
    println!("2*3={}", math(product, a, b));
    
    // 返回函数指针
    println!("return {:p}", true_maker());
    // 函数指针加上括号，就会调用该函数
    println!("return {}", true_maker()());

    // 数组的长度是编译时常量，必须在编译时确定其值
    let arr = [0; init_len()];
    println!("array length is {}", arr.len());

    let out = 42;
    // add 函数内使用外部定义的变量 out，编译器会报错
    // fn add(i: i32, j: i32) -> i32 { i + j + out }
    // 匿名函数，闭包可捕获外部变量 out
    let closure_annotated = |i: i32, j: i32| -> i32 { i + j + out };
    // 闭包自动推断输入和返回类型，个人觉得可读性不好
    let closure_inferred = |i, j| i + j + out;
    let i = 1;
    let j = 2;
    println!("closure annotated: 1+2+42={}", closure_annotated(i, j));
    println!("closure inferred: 1+2+42={}", closure_inferred(i, j));

    // 传入闭包：|| a + b
    println!("closure: 2+3={}", closure_math(|| a + b));
    // 传入闭包：|| a * b
    println!("closure: 2*3={}", closure_math(|| a * b));
    
    let result = two_times_impl();
    println!("closure: 2's two times is {}", result(2));
}