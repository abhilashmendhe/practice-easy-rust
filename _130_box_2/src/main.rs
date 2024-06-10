// 130. Box ptrs
/*

*/

#[derive(Debug)]
struct List {
    item: u32,
    next: Option<Box<List>>
}
fn main() {
    let x = Box::new(10);
    println!("{:?}",std::mem::size_of::<Box<i32>>());
    println!("{:?}",std::mem::size_of::<Box<String>>());
    println!("{:?}",std::mem::size_of::<Box<char>>());
    println!("{:?}",std::mem::size_of::<Box<Vec<String>>>());
}
