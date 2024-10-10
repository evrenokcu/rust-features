use std::sync::RwLock;

use crate::print_header;

pub(crate) fn execute() {
    print_header("RwLock<T>");
    let x = RwLock::new(MyStruct {
        name: String::from("evren"),
    });

    {
        let y = x.read().unwrap();
        println!("{:?}", &y);
    }

    {
        let mut write_access = x.write().unwrap(); // Acquire write lock
        *write_access = MyStruct {
            name: String::from("Okcu"),
        };
        // *x.write().as_mut(
        // let mut w = x.write().unwrap();
        // w.name = String::from("okcu");
        // x.write().unwrap().name = String::from("okcu");
    }

    println!("end of write lock");
    let y = x.read().unwrap();
    println!("{:?}", &y);
    let z = x.read().unwrap();
    println!("{:?}", &z);
}

#[derive(Debug)]
struct MyStruct {
    name: String,
}
