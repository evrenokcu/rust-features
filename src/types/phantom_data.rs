use std::marker::PhantomData;

use crate::print_header;

pub(crate) fn phantom() {
    print_header("phantom");
    let c: MyStruct<Cocker> = MyStruct { p: PhantomData };
    c.print();

    let c: MyStruct<Labrador> = MyStruct { p: PhantomData };
    c.print();
}

struct MyStruct<T> {
    p: PhantomData<T>,
}

impl MyStruct<Cocker> {
    fn print(&self) {
        println!("Cocker");
    }
}
impl MyStruct<Labrador> {
    fn print(&self) {
        println!("Labrador");
    }
}

struct Cocker {}

struct Labrador {}
