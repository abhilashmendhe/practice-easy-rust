// 123. Arc and mutex/RwLock
/*
    To make it thread safe we should use
    Mutex instead of RefCell, and
    Arc   instead of Rc.

    We can use RwLock instead of Mutex, only the methods needs to be called differently.
*/

use std::{sync::{Arc, Mutex, RwLock}, thread};

fn main() {
    // 1. Arc + Mutex
    // let my_num = Arc::new(Mutex::new(0));
    // // let mut handle_vec = vec![];

    // for i in 1..11 {
    //     let number_to_go_in = Arc::clone(&my_num);

    //     let handle = thread::spawn(move || {
    //         *number_to_go_in.lock().unwrap() += i;
    //     });
    //     let _ = handle.join();
    // }

    // println!("{:?}",*my_num.lock().unwrap());

    // 2. Arc + RwLock
    let my_num = Arc::new(RwLock::new(0));
    // let mut handle_vec = vec![];

    for i in 1..11 {
        let number_to_go_in = Arc::clone(&my_num);

        let handle = thread::spawn(move || {
            *number_to_go_in.write().unwrap() += i;
            *number_to_go_in.write().unwrap() += 10;
        });
        let _ = handle.join();
    }

    println!("{:?}",*my_num.read().unwrap());

    println!("{:?}",give_arc(322).read());
}

fn give_arc(input: i32) -> Arc<RwLock<i32>> {
    Arc::new(RwLock::new(input))
}