// 93. Iters/closure - 
/*
    1. .take_while() - 
    2. .skip_while() - 
    3. .cloned() - 
    4. .by_ref() -
    5. .sum() - 
*/
fn main() {
    let my_vec = (0..10).skip_while(|&x| x < 2).take_while(|&x|x <8).collect::<Vec<_>>();
    println!("{:?}",my_vec);
}
