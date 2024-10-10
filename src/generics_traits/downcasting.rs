use std::any::Any;

use crate::print_header;

pub(crate) fn execute() {
    print_header("downcasting");
    let mut list: Vec<Box<dyn Any>> = Vec::new();
    let x = String::from("evren");
    //you can not assign t to data below, because it is not static.
    //but string literal is static so it works
    //let t = x.as_str();
    list.push(Box::new(MyStruct1 { data: "evren" }));
    list.push(Box::new(MyStruct2 {}));

    for item in list.iter() {
        //as_ref is required here
        print_any(item.as_ref());
    }
}

struct MyStruct1<'a> {
    data: &'a str,
}

struct MyStruct2 {}

fn print_any(item: &dyn Any) {
    if let Some(obj) = item.downcast_ref::<MyStruct1>() {
        println!("My Struct 1");
    } else if let Some(obj) = item.downcast_ref::<MyStruct2>() {
        println!("My Struct 2");
    } else {
        println!("unknown type");
    }
}
