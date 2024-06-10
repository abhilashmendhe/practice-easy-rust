use std::fmt;


#[derive(Debug)]
struct Cat {
    name: String,
    age: u8
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"'{}' is {} years old.",self.name, self.age)
    }
}
fn main() {
    let c1 = Cat {
        name: "patchy".to_string(),
        age: 4
    };
    println!("{}",c1);
}
