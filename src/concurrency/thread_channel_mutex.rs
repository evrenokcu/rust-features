use std::{
    sync::{mpsc, Mutex},
    thread::{self, Builder, Thread},
    time::Duration,
};

pub(crate) fn thread_channel_mutex() {
    crate::print_header("thread, channel, mutex");
    let thread_handle = thread::spawn(|| {
        println!("from thread");
    });
    println!("main");
    thread_handle.join().unwrap();
    threads();
    channels();
    mutex();
    mutex_with_arc();
}
fn mutex_with_arc() {}
fn threads() {
    println!("=============threads==============");
    let thread1 = thread::spawn(|| println!("thread 1"));
    let thread2 = Builder::new()
        .name("thread 2 name".to_string())
        .spawn(|| {
            thread::sleep(Duration::from_secs(2));
            println!("thread 2");
        })
        .unwrap();
    let thread3 = thread::spawn(move || {
        let x = thread2.thread();
        println!("{:?},{:?}", x.id(), x.name());
    });
    thread3.join();
}
fn channels() {
    let (tx, tr) = mpsc::channel();
    thread::spawn(move || {
        let t = tx.send(String::from("from other thread"));
        t.unwrap();
    });

    let received = tr.recv().unwrap();
    println!("{}", received);
}
fn mutex() {
    let mutex = Mutex::new(34);
    let mut reference = mutex.lock().unwrap();
    *reference = 34;
    println!("{:#?}", reference);
}
