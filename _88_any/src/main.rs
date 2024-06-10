// 88. Iterators/closures - .any()
/*
    .any() - it checks if items present in iters/vec. Returns bool
*/
fn in_char_vec(char_vec: &Vec<char>, check: char){
    println!("Is {} inside? - {}",check, char_vec.iter().any(|&char| char == check));
} 
fn main() {
    let char_vec = ('a'..'働').collect::<Vec<_>>();
    in_char_vec(&char_vec, 'i');
}
