// 76. Iterators are collection of things that gives you one item at a time when you call .next()

/*
    .into_iter() -> iterator that owns its value = taking self
    .iter()      -> iterator of references, &self
    .iter_mut()  -> iterator of mutable references, &mut self

    // more methods
    .map -> does something to each item and passes it on.
    .for_each -> does something to each item only.
*/

fn main() {
    let vector1 = vec!  [1,2,3,5];

    // 1. .iter()
    let v1_a = vector1.iter().map(|x| x + 1).collect::<Vec<i32>>(); 
    println!("{:?}",vector1);

    // 2. .into_iter()
    let v1_b = vector1.into_iter().map(|x| x * 10).collect::<Vec<i32>>();
    println!("{:?}",v1_b);  

    // 3. .iter_mut()
    let mut vector2 = vec![10, 20, 30];
    vector2.iter_mut().for_each(|x| *x += 1);
    println!("{:?}",vector2);
}
