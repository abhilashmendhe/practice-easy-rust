// 115. Rc (reference counting)
/*
Only one owner for a piece of data.
 */

struct Book {
    name: String
}
impl Book {
    fn give_name(&self) {
        println!("Book's name: '{}'",self.name);
    }
}
fn main() {
    let mybook = Book {
        name: "My book".to_string()
    };
    mybook.give_name();
    Book::give_name(&mybook);
    // Book::give_name(&Book { name: "haha".to_string() });
}
