/**
 * 条件表达式
 */
pub fn if_expr(x: i32) -> i32 {
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
pub fn while_expr() {
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
pub fn loop_expr() {
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
pub fn for_expr() {
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
pub fn match_expr(n: i32) {
    match n {
        0 => println!("match number"),
        1...3 => println!("match range"),
        | 5 | 7 | 13  => println!("match branch"),
        n @ 42 => println!("binding {}", n),
        _ => println!("default"),
    }
}

/**
 * while let表达式
 */
pub fn while_let_pop() {
    let mut v = vec![1,2,3];
    // 动态数组的pop方法会返回Option类型，数组被取空会返回None
    // 使用match表达式，需要匹配两种情况：Some(x)和None
    while let Some(x) = v.pop() {
        println!("{}", x);
    }
}

fn main() {
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