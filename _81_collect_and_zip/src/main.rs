// 81. Iterators - .collect() and .zip() methods
/*
    .zip() - joins each elemnts of only two vectors that will result in hash map or vec of tuples and 
    tuple size is 2.
*/

// use std::collections::HashMap;

fn main() {
    let some_num = vec![0,1,2,3,4,5];
    let some_words = vec!["zero","one","two","three","four","five"];

    let num_word_hashmap = some_num
                        .into_iter()
                        .zip(some_words.into_iter())
                        // .collect::<HashMap<_,_>>();
                        .collect::<Vec<(_,_)>>();

    println!("{:?}",num_word_hashmap);
}
