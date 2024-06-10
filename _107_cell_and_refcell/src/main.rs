use std::cell::{Cell, RefCell};

#[derive(Debug)]
struct Book<'a>{
    name: Cell<&'a str>,
    author: RefCell<&'a str>
}
fn main() {
    let b1 = Book {
        name: Cell::new("Harry Potter"),
        author: RefCell::new("J.K. Rowling")
    };
    // println!("{b1:?}");

    // 1. take(), will set the value to "" or 0 depending on the type
    // b1.name.take();
    // println!("{b1:?}");
    // println!();

    // 2. borrwo_mut()
    // *b1.author.borrow_mut() = "Robert K";
    // println!("{b1:?}");

    // 3. error checking for borrow_mut()
    let refer = b1.author.borrow_mut();
    if let Ok(mut r) = b1.author.try_borrow_mut() {
        *r = "Robert K";
    } else {
        println!("Hey, it is already borrwed");
    };
    println!("{:?}",b1);
}
