// 71. type aliases and new types
// type aliases = Different names for exact same type

type MyString = String;
fn main() {
    let x = String::from("This is some writing");
    let y = MyString::from("This is some writing");
    println!("{}",x==y);
}
