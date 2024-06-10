use std::sync::Mutex;

#[derive(Debug)]
struct Book<'a> {
    name: Mutex<&'a str>,
    author: Mutex<&'a str>,
    number_sold: Mutex<u32>
}
fn main() {
    let mybook = Book {
        name: Mutex::new("Harry Potter"),
        author: Mutex::new("Jk Row"),
        number_sold: Mutex::new(1000),
    };
    
    // 1. lock and unlock
    // *mybook.author.lock().unwrap() = "J. K. Rowling"; // locks, and then immediately unlocks
    // println!("{:?}",mybook);

    // 2. check mutex exclusion
    // if you run it will run for a long time and then eventually panics the code.
    // let mut m_change1 = mybook.author.lock();
    // let mut m_change2 = mybook.author.lock();

    // 3. safe method to check if lock already applied
    let mut m_change1 = mybook.author.lock();
    drop(m_change1);
    if let Ok(mut mutex) = mybook.author.try_lock() {
        *mutex = "J.K. Rowling";
    } else {
        println!("Already Locked");
    };
    
    println!("{:#?}",mybook.author);

    // 4. incr sold_num
    for _ in 0..100 {
        *mybook.number_sold.lock().unwrap() += 1;
    }
    println!("{:?}",mybook);
}
