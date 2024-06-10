// 112 Cow_2
/*
    Cow can return vec![] or slice. 
*/

use std::borrow::Cow;

fn return_slice_or_vec<'cow>(input: &'cow [i32]) -> Cow<'cow, [i32]> {
    match input.len() {
        0..=5 => Cow::Owned(input.to_vec()),
        _ => Cow::Borrowed(input),
    }
}
fn main() {
    let x = return_slice_or_vec(&[7,8,9,10]);
    let y = return_slice_or_vec(&[1,2,3,4,5,6,7]);

    println!("{:?}",x);
    println!("{:?}",y);
}
