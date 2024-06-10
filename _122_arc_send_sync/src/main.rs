// 122. Arc, Send and Sync
/*
    We are using Rc and RefCell.
    But to make it thread safe
    we should use Mutex instead of RefCell
    and       use Arc   instead of Rc.
*/

use std::{cell::RefCell, rc::Rc, thread};

fn main() {
    let my_num = Rc::new(RefCell::new(10));

    let mut handle_vec = vec![];

    for _ in 0..10 {
        let number_to_go_in = Rc::clone(&my_num);

        let handle = thread::spawn(move || {
            *number_to_go_in.borrow_mut() += 10;
        });
    }
}
