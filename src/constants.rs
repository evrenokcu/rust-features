const X: i8 = create_constant();

pub(crate) fn constants() {
    crate::print_header("Constants");
    println!("Constant 1: {X}");
}

const fn create_constant() -> i8 {
    3
}
