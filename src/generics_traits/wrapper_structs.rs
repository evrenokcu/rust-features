use std::ops::Deref;

use crate::print_header;

pub(crate) fn execute() {
    print_header("traits for external types");
    let my_vec = MyVec(vec![1, 2, 3]);
    for item in my_vec.iter() {
        println!("{item}");
    }
}

struct MyVec<T>(Vec<T>);

impl<T> Deref for MyVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
