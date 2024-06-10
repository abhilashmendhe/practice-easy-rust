#[derive(Debug)]
struct EvenOddVec(Vec<Vec<i32>>);

impl From<Vec<i32>> for EvenOddVec {
    fn from(value: Vec<i32>) -> Self {
        let mut even_odd_vec: Vec<Vec<i32>> = vec![vec![],vec![]];
        for item in value {
            if item % 2 == 0 {
                even_odd_vec[0].push(item);
            } else {
                even_odd_vec[1].push(item);
            }
        }

        Self(even_odd_vec)
    }
}
fn main() {
    let bunch_of_numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let new_vec = EvenOddVec::from(bunch_of_numbers);
    println!("{:?}",new_vec);
}
