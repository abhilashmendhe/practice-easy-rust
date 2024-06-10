// 95. Iters/closure - .peekable()

/*
    .peekable() - 

*/
fn main() {
    let nums = vec![8,9,10,1,4];
    
    let mut number_iter = nums.iter().peekable();

    for _ in 0..3 {
        println!("I love the number {}",number_iter.peek().unwrap());
        println!("{:?}",number_iter.peek());
    }

}
