pub(crate) fn execute() {
    run_fn_once();
    run_fn();
}

fn run_fn() {
    let consumable = &String::from("ref moved");
    //below is FnOnce because consumable is moved!
    let consumer = move || consumable;
    println!("{}", consumer());
    println!("{}", consumer());
}
//consumer(); can not call an FnOnce more than once

fn run_fn_once() {
    let consumable = String::from("moved once");
    //below is FnOnce because consumable is moved!
    let consumer = move || consumable;
    println!("{}", consumer());
}
