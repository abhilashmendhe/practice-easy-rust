// 120. Threads 2
/*
    Fn - takes a reference
    FnMut - takes a mutable reference
    FnOnce - takes ownership
*/
fn main() {
    // 1. Spawing threads
    // for _ in 0..10 {
    //     let handle = std::thread::spawn(|| {
    //         println!("I am printing 1");
    //     });
    //     let handle2 = std::thread::spawn(|| {
    //         println!("I am printing 2");
    //     });
    //     handle.join();
    //     handle2.join();
    // }

    // 2. spawing threads with move closure
    for i in 0..10 {
        let handle = std::thread::spawn(move || {
            println!("I am thread {}",i);
        });
        // handle.join();
        println!("{}",i);
    }
}
