// 94. chunks
/*

*/
fn main() {
    let num_vec = vec![1,2,3,4,5,6,7,8,9,10];

    //1. .chunks()
    for chunk in num_vec.chunks(4) {
        println!("{:?}",chunk);
    }

    //2. .windows()
    for window in num_vec.windows(4) {
        println!("{:?}",window);
    }

    //3. .match_indices()
    let rules = "Rule number 1: No fighting. Rule number 2: Go to bed at 8 pm. Rule number 3: Wake up at 6 am.";
    let rule_locations = rules.match_indices("Rule").collect::<Vec<(_, _)>>(); // This is Vec<usize, &str> but we just tell Rust to do it
    println!("{:?}", rule_locations);
}
