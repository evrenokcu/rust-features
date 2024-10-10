pub(crate) fn execute() {
    let closure = |left: fn() -> i32, right: fn() -> i32| left() + right();
    let x = &closure(|| 1, || 2);
    println!("sum with closure:{x}");
}
