use std::fmt::Debug;

use crate::print_header;

pub(crate) fn execute() {
    print_header("blanket traits");
    let s = MyStruct::new("blanket trait");
    s.evren_print();
}
trait MyBlanketTrait {
    fn evren_print(&self);
}

impl<T: Debug> MyBlanketTrait for T {
    fn evren_print(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
struct MyStruct {
    name: String,
}
impl MyStruct {
    fn new(s: &str) -> Self {
        Self { name: s.into() }
    }
}
