// 127. Impl trait
/*
    We will return closures from functions.
*/


fn returns_a_closure(input: &str) -> impl FnMut(i32) -> i32 {
    match input {
        "double" => |mut number| {
            number *= 2;
            println!("Your number is {}",number);
            number
        },
        "triple" => |mut number| {
            number *= 3;
            println!("Triple!. The number is {}",number);
            number
        },
        _ => |number| {
            println!("I only understand triple or double!");
            number
        }
    }
}
fn main() {

    let mut closure1 = returns_a_closure("double");
    let mut closure2 = returns_a_closure("triple");
    let mut closure3 = returns_a_closure("hahhah");

    closure1(10);
    closure2(10);
    closure3(10);
}