use std::thread;
use std::time::Duration;

fn dispach_more() {
    thread::spawn(|| {
        for i in 1..5 {
            println!("hello from more thread {}", i);
        }
    })
    .join()
    .unwrap();
}

fn dispach() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hello from thread {}", i);
            dispach_more();
        }
    })
    .join()
    .unwrap();
}

fn main() {
    dispach();
}
