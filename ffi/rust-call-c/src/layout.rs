use std::mem;

// 默认布局
struct DefaultStruct {
    first: i8,
    second: i16,
    third: i8
}

// 默认布局，对齐方式降低到 1
#[repr(packed(1))]
struct PackedStruct {
    first: i8,
    second: i16,
    third: i8
}

// C 布局
#[repr(C)]
struct CStruct {
    first: i8,
    second: i16,
    third: i8
}

// C 布局, 对齐方式升高到 4
#[repr(C, align(4))]
struct AlignedStruct {
    first: i8,
    second: i16,
    third: i8
}

// C 布局的元组结构体
#[repr(C)]
struct TupleStruct(i8, i16, i8);

// C 布局，重新调整字段的顺序可以缩小类型大小
#[repr(C)]
struct FieldStructOptimized {
    first: i8,
    third: i8,
	second: i16
}

// 联合类型的大小等于其字段类型的最大值
#[repr(C)]
union ExampleUnion {
    smaller: i8,
    larger: i16
}


fn main() {
    assert_eq!(mem::align_of::<DefaultStruct>(), 2);
    assert_eq!(mem::size_of::<CStruct>(), 6);
    assert_eq!(mem::align_of::<CStruct>(), 2);
    
    assert_eq!(mem::align_of::<PackedStruct>(), 1);
    assert_eq!(mem::align_of::<AlignedStruct>(), 4);
    
    assert_eq!(mem::size_of::<FieldStructOptimized>(), 4);
    
    assert_eq!(mem::size_of::<TupleStruct>(), 6);

    assert_eq!(mem::size_of::<ExampleUnion>(), 2);
}