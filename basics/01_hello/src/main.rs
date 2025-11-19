/// ========== begin variables ==========
const INT1: i32 = 1;
const BIT2: u32 = 1 << 1;
const STRING: &'static str = "String";

fn variables() {
    // 在函数中访问常量
    println!("{:?}", INT1);
    println!("{:?}", BIT2);
    println!("{:?}", STRING);

    // 报错！不能修改一个 `const` 常量
    // INT1 = 5;
    let a = 1;
    // 默认不可变绑定，不能重新赋值
    // a = 2;
    // 可使用mut关键字，创建可变绑定
    println!("a {:p}", &a);
    let mut b = 2;
    println!("b: {}", b);
    b = 3;
    println!("b was changed: {}", b);
    
    // 变量遮蔽：连续定义同名变量
    let s = "Hello Rust";
    println!("s is {}", s);
    let s = "Hello World";
    // 变量生命周期，词法作用域
    {
        let s = "Hello Rust";
        println!("s is {}", s);
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
/// ========== end variables ==========


/// ========== begin primitives ==========
fn basic_example() {
    // 布尔类型
    let a_boolean: bool = true;
    println!("a_boolean = {}", a_boolean);

    // 数值类型
    let a_float: f32 = 1.0;  // 变量常规声明
    let an_integer   = 6i16; // 变量后缀声明
    println!("a_float = {}", a_float);
    println!("an_integer = {}", an_integer);

    // 可根据上下文自动推断类型
    let mut inferred_type = 8; // 根据下一行的赋值推断为i64类型
    println!("inferred_type = {}", inferred_type);
    inferred_type = 64i64;
    println!("inferred_type = {}", inferred_type);

    // 无法类型推断时，按默认方式取类型
    let default_float   = 2.0; // 浮点数值为f64
    let default_integer = 5;   // 整型数值为i32
    println!("default_float = {}", default_float);
    println!("default_integer = {}", default_integer);

    // 字符类型
    let a_char: char = 'a';
    println!("a_char = {}", a_char);
}

fn array_example () {
    // 创建数组方式一：[x, y, z]
    let arr: [i32; 3] = [1, 2, 3];
    let mut mut_arr = [4, 5, 6];
    mut_arr[0] = 0;

    assert_eq!(3, arr[2]);
    assert_eq!(0, mut_arr[0]);
    // 这个循环输出: 1 2 3
    for x in &arr {
        print!("{} ", x);
    }
    println!();

    // 创建数组方式二：[x; N]
    let mut array: [i32; 3] = [0; 3];
    array[1] = 1;
    array[2] = 2;

    assert_eq!([1, 2], &array[1..]);
    // 元素个数小于等于32的数组，实现了`trait IntoIterator`
    // 这个循环输出: 0 1 2
    for x in &array {
        print!("{} ", x);
    }
    println!();

    let array: [i32; 33] = [0; 33];
    // error[E0277]: `&[i32; 33]` is not an iterator
//    for x in &array {
//        print!("{} ", x);
//    }
    // 通过调用slice方法将数组强制类型转换为slice
    for x in array.iter() {
        print!("{} ", x);
    }
    println!();
}

fn tuple_example() {
    let tup: (u8, i32, f64) = (1, 100, 1.1314);
    let (x, y, z) = tup;
    let f_number = tup.2;
    let one_tup = (1.1,);
    println!("elements in tuple {},{},{}", x, y, z);
    println!("third element in tuple {}", f_number);
    println!("one element in tuple {}", one_tup.0);
}

fn struct_example() {
    struct Person {
        age: u8,
        is_child: bool,
    }
    struct OnePerson(u8, bool);
    struct UnitStruct;
    let alice = Person {age: 10, is_child: true};
    let bob = OnePerson(32, false);
    let x = UnitStruct;
    println!("alice age {} is child {}", alice.age, alice.is_child);
    println!("bob age {} is child {}", bob.0, bob.1);
    println!("unit struct {:p}", &x);

    impl Person {
        fn create_person(age: u8, is_child: bool) -> Person {
            Person{age, is_child}
        }
        fn check_child(&self) -> bool {
            if self.is_child && self.age < 18 {
                return true;
            } else {
                return false;
            }
        }
    }
    let peter = Person::create_person(33, true);
    println!("peter age {} is child {}", peter.age, peter.is_child);
    println!("peter is child {}", peter.check_child());
}

fn enum_example() {
    enum Number {
        Integer(i64),
        Float {
            inner: f64
        },
    }
    let a = Number::Integer(10);
    let b = Number::Float {
        inner: 3.14
    };
    match a {
        Number::Integer(n) => println!("a is integer: {}", n),
        Number::Float {inner} => println!("a is float: {}", inner),
    }
    if let Number::Float { inner } = b {
        println!("b is float: {}", inner);
    }
}

fn primitives() {
    basic_example();
    array_example();
    tuple_example();
    struct_example();
    enum_example();
}
/// ========== end primitives ==========


/// ========== begin functions ==========
/// 函数作为参数
pub fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    op(a,b)
}
/// 求和函数
fn sum(a: i32, b: i32) -> i32 {
    // 表达式，无分号返回求值结果
    a + b
    // return 仅在需要提前返回时使用
    // return a + b;
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

fn answer() -> () {
    // 声明语句
    let a = 40;
    let b = 2;
    // 表达式语句：以分号结尾的表达式
    // println! 宏语句：名字以叹号结尾，并像函数一样被调用
    println!("40 + 2 = {}", sum(a, b));
}

fn functions() {
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

    answer();
}
/// ========== end functions ==========


/// ========== begin control_flow ==========
/**
 * 条件表达式
 */
fn if_expr(x: i32) -> i32 {
    let n = if x < 10 && x > -10 {
        10 * x
    } else {
        // 如果传入奇数，返回类型为i32，编译器是否会报错？
        x / 2
    };
    return n;
}

/**
 * 循环表达式 while
 */
fn while_expr() {
    let mut n = 1;
    while n < 16 {
        if n % 15 == 0 {
            println!("3 and 5‘s multiple {}", n);
        } else if n % 5 == 0 {
            println!("5‘s multiple {}", n);
        }
        n += 1;
    }
}

/**
 * 循环表达式 loop
 */
fn loop_expr() {
    let mut n = 1;
    loop {
        if n % 15 == 0 {
            println!("3 and 5‘s multiple {}", n);
        } else if n % 3 == 0 {
            println!("3‘s multiple {}", n);
        } else if n > 16 {
            break;
        }
        n += 1;
    }
}

/**
 * 循环表达式 for...in
 */
fn for_expr() {
    for n in 1..16 {
        if n % 15 == 0 {
            println!("3 and 5‘s multiple {}", n);
        } else if n % 5 == 0 {
            println!("5‘s multiple {}", n);
        }
    }
}

/**
 * match表达式
 */
fn match_expr(n: i32) {
    match n {
        0 => println!("match number"),
        1..=3 => println!("match range"),
        | 5 | 7 | 13  => println!("match branch"),
        n @ 42 => println!("binding {}", n),
        _ => println!("default"),
    }
}

/**
 * while let表达式
 */
fn while_let_pop() {
    let mut v = vec![1,2,3];
    // 动态数组的pop方法会返回Option类型，数组被取空会返回None
    // 使用match表达式，需要匹配两种情况：Some(x)和None
    while let Some(x) = v.pop() {
        println!("{}", x);
    }
}

fn control_flow() {
    let x = 13;
    // Rust编译器根据上下文，会将结果截取
    println!("result={}", if_expr(x));

    while_expr();
    loop_expr();
    for_expr();

    let mut n = 2;
    match_expr(n);
    n = 5;
    match_expr(n);
    n = 42;
    match_expr(n);
    n = 100;
    match_expr(n);

    while_let_pop();
}
/// ========== end control_flow ==========

fn main() {
    println!("Hello, world!");
    variables();
    primitives();
    functions();
    control_flow();
}
