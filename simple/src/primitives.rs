fn array_example () {
    let arr: [i32; 3] = [1, 2, 3];
    let num = arr[2];
    let mut mut_arr = [4, 5, 6];
    mut_arr[0] = 0;
    println!("array's size {}", arr.len());
    println!("last element in array {}", num);
    println!("first element in array {}", mut_arr[0]);
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

fn basic_example() {
    // 布尔类型
    let a_boolean: bool = true;

    // 数值类型
    let a_float: f32 = 1.0;  // 变量常规声明
    let an_integer   = 6i16; // 变量后缀声明

    // 可根据上下文自动推断类型
    let mut inferred_type = 8; // 根据下一行的赋值推断为i64类型
    inferred_type = 64i64;

    // 无法类型推断时，按默认方式取类型
    let default_float   = 2.0; // 浮点数值为f64
    let default_integer = 5;   // 整型数值为i32

    // 字符类型
    let a_char: char = 'a';
}

fn main() {
    array_example();
    tuple_example();
    struct_example();
    enum_example();
    basic_example();
}