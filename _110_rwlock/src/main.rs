// 110. RwLock -> read/write lock
/*
Instead of lock() method, we have read() and write() methods.
    read() - many .
    write() - only one.
*/
use std::sync::RwLock;

#[derive(Debug)]
struct Book<'a> {
    title: RwLock<&'a str>,
    author: RwLock<&'a str>,
    year: RwLock<u32>
}

fn main() {
    let mybook = Book {
        title: RwLock::new("Harry Potter"),
        author: RwLock::new("J.K Row"),
        year: RwLock::new(1991)
    };
    // 1. example with Mutex.lock()
    // let mut title = mybook.title.lock().unwrap();
    // let mut author = mybook.author.lock().unwrap();
    // let mut year = mybook.year.lock().unwrap();
    // println!("{:?}",mybook);

    // 2. example with .read()
    // let title = mybook.title.read().unwrap();
    // let author = mybook.author.read().unwrap();
    // let year = mybook.year.read().unwrap();
    // println!("{:?}",mybook);

    // 3. with .write()
    let mut title = mybook.title.write().unwrap();
    let mut author = mybook.author.write().unwrap();
    let mut year = mybook.year.write().unwrap();
    *author="J. K. Rowling";
    drop(author); // after manipulating the data, remember to unlock by calling drop() function.
    println!("{:?}",mybook);
}
