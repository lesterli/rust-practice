const INT1: i32 = 1;
const BIT2: u32 = 1 << 1;
const STRING: &'static str = "String";

static RUST: &'static str = "Rust";
static mut MUT_INT: i32 = 1;

#[derive(Debug)]
struct BitString<'a> {
    mybit: u32,
    mystring: &'a str,
}

const BIT_STRING: BitString<'static> = BitString {
    mybit: BIT2,
    mystring: STRING,
};

fn is_big(n: i32) -> bool {
    // 在一般函数中访问常量
    n > INT1
}

fn main() {
    // 在 main 函数中访问常量
    println!("{:?}", INT1);
    println!("{:?}", BIT2);
    println!("{:?}", STRING);
    println!("{:#?}", BIT_STRING);

    // 报错！不能修改一个 `const` 常量
    INT1 = 5;
    let n = 4;
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });


    MUT_INT = 5;


    let a = 1;
    // 默认不可变绑定，不能重新赋值
    // a = 2;
    // 可使用mut关键字，创建可变绑定
    println!("a {:p}", &a);
    let mut b = 2;
    b = 3;
    println!("b was changed: {}", b);
    
    // 变量遮蔽：连续定义同名变量
    let s = "Hello Rust";
    let s = "Hello World";
    // 变量生命周期，词法作用域
    {
        let s = "Hello Rust";
    }
    println!("s is {}", s);

    pub fn tmp() -> i32 {
        return 1;
    }
    // 借用操作符&，获取表达式内存地址
    let x = &tmp();
    // 值表达式不能出现在位置上下文中，E0070
    // tmp() = *x;
    println!("x is memory address: {:p}", x);
    // 声明动态数组，vec!
    let mut c = vec![1,2,3];
    // 使用借用操作符&，得到引用类型
    let d = &mut c;
    d.push(4);
    println!("{:?}", d);
    // 字面常量是值表达式，在位置上下文中求值创建临时值
    let e = &42;
    // 使用解引用操作符*，取得引用中的值
    println!("reference e's value is {}", *e)
}