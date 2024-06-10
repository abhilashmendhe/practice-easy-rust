// RefCell is cousin of Cell
// It dynamicall check borrow rules in runtime

use std::cell::RefCell;

#[derive(Debug)]
struct  User {
    id: u32,
    year_registerd: u32,
    username: String,
    active: RefCell<bool>
}

fn main() {
    let u1 = User {
        id: 1,
        year_registerd: 2020,
        username: "abh123".to_string(),
        active: RefCell::new(true)
    };
    println!("{:?}",u1);
    
    // let mut reference = u1.active.borrow_mut();

    // below will give you panic, because already borrowed
    // let mut reference2 = u1.active.borrow_mut();

    // *reference = false;
    // std::mem::drop(reference);
    // dbg!(&u1);

    // easy way
    *u1.active.borrow_mut() = false;
    dbg!(&u1);
}
