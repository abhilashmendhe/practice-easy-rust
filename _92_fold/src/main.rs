// 92. Iters/closure - .fold()

/*
    .fold() 
*/

fn main() {

    // 1. .fold()
    let some_numbers = vec![9,6,9,10,11];
    println!("{}",some_numbers.iter().fold(0,|total_so_far, next_num| total_so_far+next_num));

    // 2. 
    let a_string = "I don't have any dishes in me.";
    println!("{}",a_string.chars().fold("-".to_string()  , |mut string_so_far, nex_char| {
        string_so_far.push(nex_char);
        string_so_far.push('-');
        string_so_far
    }));
}

