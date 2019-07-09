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

fn main() {
    let x = 13;
    // Rust编译器根据上下文，会将结果截取
    println!("result={}", if_expr(x));

    while_expr();
    loop_expr();
    for_expr();
}