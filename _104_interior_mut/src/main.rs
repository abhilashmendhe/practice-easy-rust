// #[derive(Debug)]
// struct SomeStruct {
//     r_field: u8,
//     s_filed: u8
// }

use std::cell::Cell;

// with Cell we can change immutable struct value but they are not thread safe..
#[derive(Debug)]
struct SomeStruct {
    r_field: u8,
    s_filed: Cell<u8>
}

fn main() {
    let mystruct = SomeStruct{
        r_field: 10,
        s_filed: Cell::new(11)
    };
    println!("{:?}",mystruct);
    // mystruct.s_filed = 123;
    mystruct.s_filed.set(123);
    println!("{:?}",mystruct);
}
