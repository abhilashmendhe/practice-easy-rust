// 97. dbg() macro

fn main() {
    // println!("Hello, world!");

    // dbg!(10);
    // // eprintln!("{}",);

    let mut my_num = 9;
    my_num +=10;

    let new_vec = vec![8,9,10];
    let double_vec = new_vec.iter().map(|x| x * 2).collect::<Vec<_>>();
    println!("{:?}",double_vec);
}
