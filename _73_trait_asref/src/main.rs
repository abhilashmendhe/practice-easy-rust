// 73. AsRef trait

use std::fmt::Display;
use std::convert::AsRef;

fn print_it<T: AsRef<str> + Display>(input: T) {
    println!("{}",input);
}
fn main() {
    print_it("Please print this".to_string());
}
