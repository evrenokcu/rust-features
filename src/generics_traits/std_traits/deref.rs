use std::ops::{Deref, DerefMut};

struct MyStruct {
    name: String,
}

pub(crate) fn deref() {
    deref_to_string();
    my_box();
}

fn my_box() {
    let boxed = MyBox(MyStruct {
        name: "boxed my struct".to_string(),
    });
    println!("boxed my struct:{}", boxed.name);
}

impl DerefMut for MyStruct {
    fn deref_mut(&mut self) -> &mut Self::Target {
        todo!()
    }
}

fn deref_to_string() {
    let my = MyStruct {
        name: "evren".to_string(),
    };
    print_string(&my);
}

impl Deref for MyStruct {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.name
    }
}

fn print_string(s: &String) {
    println!("deref: {}", s)
}
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
