use std::borrow::Borrow;

pub(crate) fn borrow() {
    crate::print_sub_header("borrow strings");

    let s1: String = String::from("String");
    let s2 = "str";

    print_borrow_str("evren");
    print_borrow_str("Evren".to_string());
    print_borrow_str(s1);
    print_borrow_str(s2);
}

fn print_borrow_str<T: Borrow<str>>(s: T) {
    let s: &str = s.borrow();
    println!("borror String:{s}");
}
