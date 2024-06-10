// 91. Iters/closures -  .cycle() 
/*

*/
fn main() {
    // 1. .cycle()
    // let even_odd = vec!["even","odd"];
    // let even_odd_vec = (0..6)
    //                 .zip(even_odd.into_iter().cycle())
    //                 .collect::<Vec<_>>();
    // println!("{:?}",even_odd_vec);

    // 2. .cycle()
    let ten_chars = ('a'..).take(10).collect::<Vec<char>>();
    let skip_then_ten_chars = ('a'..).skip(1300).take(10).collect::<Vec<char>>();

    // let all_chars = ('a'..).collect::<Vec<char>>();
    // println!("{:?}",all_chars);
}
