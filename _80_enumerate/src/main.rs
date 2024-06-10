// 80. Iterators - .enumerate()
/*
    .enumerate() - Gives index of item along with an item.
    .for_each()  - is basically for loop
*/
fn main() {
    let num_vec = vec![10,9,8];

    // 1. using for loop
    // for (idx, item) in  num_vec.iter().enumerate() {
    //     println!("idx: {}, val: {}",idx, item);
    // }
    
    // 2. using for_each()
    // num_vec.iter()
    //         .enumerate()
    //         .for_each(|(idx, num)| println!("idx: {}, val: {}",idx, num));

    // 3. using map()
    let new_vec: Vec<_> = num_vec.iter()
            .enumerate()
            .map(|(idx, num)| (idx, num*idx)).collect();
    
    println!("{:?}",new_vec);

}
