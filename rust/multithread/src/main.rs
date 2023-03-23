use std::{thread, vec};
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    
    for i in 1..5 {
        println!("hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();


    // move closure
    let v = vec![1,2,3];
    // error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
    // ^^ may outlive borrowed value `v`
    let handle = thread::spawn(move || {
        println!("vector: {:?}", v);
    });

    handle.join().unwrap();
}
