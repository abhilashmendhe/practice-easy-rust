// 74. Chaining methods

fn main() {
    // let mut new_vec:Vec<i32> = Vec::new();
    // let mut counter = 1;

    // while counter < 11 {
    //     new_vec.push(counter);
    //     counter += 1;
    // }
    // println!("{:?}",new_vec);

    // Chaining methods

    // 1.  collect methods
    // let new_vec = (1..=10).collect::<Vec<i32>>();
    // println!("{:?}",new_vec);

    // 2. chaining multi methods
    // skip(n) - skip first n elements
    // take(n) - take n elements
    let my_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let new_vec = my_vec
                        .into_iter()
                        .skip(3)
                        .take(4)
                        .collect::<Vec<i32>>();
    println!("{:?}",new_vec);
}
