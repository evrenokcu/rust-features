use std::{fs::File, path::*};
pub(crate) fn as_ref() {
    crate::print_sub_header("as ref");
    let c = MyContainer {
        name: "evren".to_string(),
    };

    //if T:AsRef<X> is implemented then
    // &T:AsRef<X> is also implemented

    print_string_ref(&c);
    print_string_ref2((&c).as_ref());
    print_string_ref_as_trait(&c);
    print_string_ref(c);
}

struct MyContainer {
    name: String,
}

// impl AsRef<String> for MyContainer {
//     fn as_ref(&self) -> &String {
//         &self.name
//     }
// }

impl AsRef<String> for MyContainer {
    fn as_ref(&self) -> &String {
        &self.name
    }
}

fn print_string_ref<T: AsRef<String>>(par: T) {
    println!("AsRef:{}", par.as_ref());
}
fn print_string_ref2(str: &String) {
    println!("as ref: {}", str);
    //File::open(path)
}

fn print_string_ref_as_trait(str: impl AsRef<String>) {
    println!("as ref implementation with trait:{}", str.as_ref())
}

//impl<T: ?Sized, U: ?Sized> const AsRef<U> for &T {}
