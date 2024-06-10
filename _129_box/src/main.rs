// 129. Box
/*
    Box is a smart pointer that has its data on heap.
    Box - 8 bytes ptr.
*/
fn main() {
    // 1. first example
    // let x = 1;
    // let x_box = Box::new(x);
    // println!("{}, {:?}",x, x_box);

    // 2. Allocating data on stack vs heap
    // let x = [8; 3_000_000]; // overflowed
    // println!("{:?}",x.len());

    let x = vec![8;3_000_000].into_boxed_slice();


}
