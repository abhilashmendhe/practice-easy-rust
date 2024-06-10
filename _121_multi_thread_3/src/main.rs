// 121. Thread 3
/*

*/
use std::thread;
fn main() {
    // 1. demo
    // for i in 0..10 {
    //     let handle = thread::spawn(move ||{
    //         // println!("I am thread: {i}");
    //         if i%2==0 {
    //             panic!();
    //         } else {
    //             i
    //         }
    //     });
    //     println!("{:?}",handle.join());
    //     // let _ = handle.join();
    // }

    // 2. pushing handles in vec
    let mut handles = vec![];
    for i in 0..10 {
        let handle = thread::spawn(move || {
            println!("I am inside thread {i}");
        });
        handles.push(handle);
    }
    for h in handles {
        let _ = h.join();
    }
}
