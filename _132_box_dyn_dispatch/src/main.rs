// 132.  Box with dyn dispatch
/*
    dynamic dispatch - choosing types at runtime
*/

use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ErrorOne;
impl Error for ErrorOne{}
impl fmt::Display for ErrorOne {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "You got first error!")
    }
}

#[derive(Debug)]
struct ErrorTwo;
impl Error for ErrorTwo{}
impl fmt::Display for ErrorTwo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "You got second error!")
    }
}

// impl Error -> static
// You get exptected struct ErrorOne found ErrorTwo, becuase of static impl 
// So here instead of impl Error -> Box<dyn Error>
// fn returns_errors(input: u8) -> Result<String, impl Error> {
    fn returns_errors(input: u8) -> Result<String, Box<dyn Error>> {
    match input {
        0 => Err(Box::new(ErrorOne)),
        1 => Err(Box::new(ErrorTwo)),
        _ => Ok("Looks fine.".to_string())
    }
}
fn main() {
    let vec_of_u8s = vec![0_u8, 1, 80];
    for num in vec_of_u8s {
        match returns_errors(num) {
            Ok(inp) => println!("{}", inp),
            Err(message) => println!("{}",message),
        }
    }
}
