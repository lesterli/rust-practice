pub fn trait_object() {
    #[derive(Debug)]
    struct Foo;
    trait Bar {
        fn baz(&self);
    }
    impl Bar for Foo {
        fn baz(&self) { println!("{:?}", self) }
    }
    fn static_dispatch<T>(t: &T) where T:Bar {
        t.baz();
    }
    fn dynamic_dispatch(t: &Bar) {
        t.baz();
    }
    let foo = Foo;
    static_dispatch(&foo);
    dynamic_dispatch(&foo);
}


pub fn impl_trait(){
    use std::fmt::Debug;
    pub trait Fly {
        fn fly(&self) -> bool;
    }
    #[derive(Debug)]
    struct Duck;
    #[derive(Debug)]
    struct Pig;
    impl Fly for Duck {
        fn fly(&self) -> bool {
            return true;
        }
    }
    impl Fly for Pig {
        fn fly(&self) -> bool {
            return false;
        }
    }
    fn fly_static(s: impl Fly+Debug) -> bool {
        s.fly()
    }
    fn can_fly(s: impl Fly+Debug) -> impl Fly {
        if s.fly(){
            println!("{:?} can fly", s);
        }else{
            println!("{:?} can't fly", s);
        }
        s
    }
    fn dyn_can_fly(s: impl Fly+Debug+'static) -> Box<dyn Fly> {
        if s.fly(){
            println!("{:?} can fly", s);
        }else{
            println!("{:?} can't fly", s);
        }
        Box::new(s)
    }
    let pig = Pig;
    assert_eq!(fly_static(pig), false);
    let duck = Duck;
    assert_eq!(fly_static(duck), true);

    let pig = Pig;
    can_fly(pig);
    let duck = Duck;
    can_fly(duck);

    let duck = Duck;
    dyn_can_fly(duck);
}

fn main() {
    trait_object();
    impl_trait();
}