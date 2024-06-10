use std::borrow::Cow;

// 111. Cow - Clone on write
/*
Cow returns either String or &str. We can use when we don't know what could be returned either
String or &str.
 */
fn modulo_3(input: u8) -> Cow<'static, str> {
    match input % 3 {
        0 => "Remainder is 0".into(),
        1 => "Remainder is 1".into(),
        remainder => format!("Remainder is {}", remainder).into()
    }
}
 fn main() {
    for number in 1..=10 {
        match modulo_3(number) {
            Cow::Borrowed(message) => println!("{} went in. The Cow is borrowed with the message: {}",number,message),
            Cow::Owned(message) => println!("{} went in. The Cow is owned with message: {}",number,message),
        }
    }
}
