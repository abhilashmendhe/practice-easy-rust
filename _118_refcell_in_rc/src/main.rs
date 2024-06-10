use std::{cell::RefCell, rc::Rc};

// 118. RefCell in Rc
/*
    Rc - Multiple owners
    RefCell - Modify content of variable even if not mutable/immutable
*/
fn add_bang(input: Rc<RefCell<String>>) {
    let mut my_ref = input.borrow_mut();
    println!("Numbers of owners: {}",Rc::strong_count(&input));
    my_ref.push('!');
}
fn main() {
    let mystring = Rc::new(RefCell::new("I am a string".to_string()));
    add_bang(Rc::clone(&mystring));
    println!("Numbers of owners(main): {}",Rc::strong_count(&mystring));
    println!("{:?}",mystring);
}
