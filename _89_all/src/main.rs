// 89. Iters/closures - .all()
// .all() - checks if all elements in iters/vec with some condition returns bool
fn main() {
    let smaller_vec = ('A'..'z').collect::<Vec<char>>();
    println!("All alphabetic? {}", smaller_vec.iter().all(|&x| x.is_alphabetic()));
    println!("All less than the character 행? {}", smaller_vec.iter().all(|&x| x < '행'));
}
