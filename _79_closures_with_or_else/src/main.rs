// 79. closures

// .unwrap_or_else
fn main() {

    let x: Option<String> = None;
    // println!("{:?}",x.unwrap_or("Nothing inside.".to_string()));
    println!("{:?}",x.unwrap_or_else(|| {
        let mut my_string = "I am a string".to_string();
        my_string.push('!');
        my_string
    }));
}
