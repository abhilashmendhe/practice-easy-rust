// 82. Iterators - char_indices()

/*

*/
fn main() {
    // 1. Basic example of .chars()
    // println!("{:?}","Hello There".chars().collect::<String>());

    // 2. .char_indices()
    // let numbers_together = "140399923481800622623218009598281";
    // numbers_together
    //     .char_indices()
    //     .for_each(|(idx, num)| {
    //         match (idx % 3, num) {
    //             (0 | 1, num) => print!("{}",num),
    //             _ => print!("{}\t",num),
    //         }
    //     });

    // 3. .for_each()
    let my_vec = vec![8,9,10];
    print!("{:?}", my_vec.iter().for_each(|_| println!("We're not doing anything with these nums.")));
}
