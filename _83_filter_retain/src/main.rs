// 83. Iterator - .filter() and .retain()
//     Closures - Fn, FnOnce, FnMut

/*
    .filter() - Keep anything that returns true.
    .retain() - Works on mutable iterators. We can condition on it to 
    retain those values present inside iterators/vecs

    Closures has 3 traits {Fn, FnOnce, FnMut}
    1) Fn - takes by reference (&self)
    2) FnMut - takes by mutable reference (&mut self)
    3) FnOnce - takes by value (self)

*/
fn main() {

    // 1. .filter()
    // let months = vec!["January", "February", "March", "April", "May", "June", 
    //         "July", "August", "September", "October", "November", "December"];

    // let filtered_months = months
    //                             .into_iter()
    //                             .filter(|month| month.len() < 5)
    //                             .filter(|month| month.contains("u"))
    //                             .collect::<Vec<&str>>(); 

    // println!("{:?}",filtered_months);

    // 2. .retain()
    let mut my_vec = vec![1,2,3,4,5,6,7];
    my_vec.retain(|item| item % 2 == 0);
    println!("{:?}",my_vec);

    // 3. Closures with the traits


}
