use std::{
    sync::{Arc, Mutex},
    thread,
};

use crate::print_header;

fn arc() {
    let arc = Arc::new(0);
    let a2 = arc.clone();
    let a3 = arc.clone();
}
fn mutex() {
    print_header("mutex");
    let m = Mutex::new(23);
    {
        let mut x = m.lock().unwrap();
        *x += 2;
    }
    println!("value of mutex {:?}", m.lock().unwrap());
}

pub(crate) fn arc_mutex() {
    print_header("arc and mutex");
    arc_with_mutex();
    mutex();
}

fn arc_with_mutex() {
    let mut counter = Arc::new(Mutex::new(0));
    let mut thread_list = Vec::new();

    for _ in 0..5 {
        let counter_ref = Arc::clone(&counter);
        let thread = thread::spawn(move || {
            println!("thread message");
            let mut counter = counter_ref.lock().unwrap();
            *counter = *counter + 1;
        });
        thread_list.push(thread);
    }
    for thread in thread_list {
        thread.join();
    }
    // thread::sleep(Duration::from_secs(1));
    println!("{}", counter.lock().unwrap());
}
