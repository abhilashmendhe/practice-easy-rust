//108. Mutex
/* 
Locking a variable unless you change it.
At most one at a time can acces a data.
 */

use std::sync::Mutex;
fn main() {
    let my_mutex = Mutex::new(10);
    println!("{:?}",my_mutex);

    // 1. hard way
    // let mut mutex_changer = my_mutex.lock().unwrap();
    // println!("{:?}",my_mutex);
    // println!("{:?}",mutex_changer);
    // *mutex_changer = 123;
    // drop(mutex_changer); // this drops the lock or basically unlock. Not anymored locked.
    // // println!("{:?}",mutex_changer);
    // println!("{:?}",my_mutex);

    // 2. easy way
    *my_mutex.lock().unwrap() = 123;
    println!("{:?}",my_mutex);
}
