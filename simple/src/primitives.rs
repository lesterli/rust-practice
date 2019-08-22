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

fn main() {
    array_example();
    tuple_example();
    struct_example();
    enum_example();
}