use std::cell::Cell;

struct SomeStruct {
    regular_field: u8,
    special_field: Cell<u8>,
}

fn main() {
    let my_struct = SomeStruct {
        regular_field: 0,
        special_field: Cell::new(1),
    };

    let new_value = 100;
//    my_struct.regular_field = new_value; // ERROR: `my_struct`是不可变的

    my_struct.special_field.set(new_value); // WORKS: `special_field`是`Cell`类型的，它是可变的
    assert_eq!(my_struct.special_field.get(), new_value);
}