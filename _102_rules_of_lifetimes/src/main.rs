// 101. rules of lifetimes
fn return_str<'a,'b>(input1: &'a str, input2: &'b str) -> &'a str {
    input1
}
fn main() {

    return_str("hi", "yo");
}
