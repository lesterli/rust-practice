/**
 * 找数字列表中最大的数字
 */
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/**
 * 泛型实现找列表中最大值
 */
fn generics_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    &largest
}

// 结构体泛型
struct Point<T, U> {
    x: T,
    y: U,
}

// 方法中定义泛型，必须在 impl 后面声明 T
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    // 结构体定义中的泛型类型参数并不总是与结构体方法签名中使用的泛型是同一类型
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

/**
 * trait
 */
trait Arithmetic {
    fn add(&self) -> i32;
}

struct MyPoint {
    x: i32,
    y: i32,
}

// 为类型实现trait
impl Arithmetic for MyPoint {
    fn add(&self) -> i32 {
        self.x + self.y
    }
}

// trait 作为参数
fn sum(item: impl Arithmetic) {
    println!("sum {}", item.add());
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = generics_largest(&char_list);
    println!("The largest char is {}", result);

    let p = Point { x: 5, y: 1.0 };
    println!("p.x = {}", p.x());

    let p2 = Point { x: "Hello", y: 'c'};
    let p3 = p.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let mp = MyPoint { x: 5, y: 10 };
    println!("mp.x + mp.y: {}", mp.add());

    sum(mp);
}