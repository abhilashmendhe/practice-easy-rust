use std::thread;

// 119. Multi-thread 1
/*

*/
fn main() {
    thread::spawn(|| {
        println!("Hello there. I am in thread 1.");
    });
    thread::spawn(|| {
        println!("Hello there. I am in thread 2.");
    });

    for _ in 0..10000 {

    }
}
