// 125. Closures in func.
/*

*/
#[derive(Debug)]
struct Book {
    name: String,
    year: u32
}
impl Book {
    fn print_name(&self) {
        println!("The book's name is {}",self.name);
    }
    fn change<F>(&mut self, f: F)
    where F: Fn(&mut Book) 
    {
        f(self);
    }
}
fn main() {
    let mut mybook = Book {
        name: "Harry Potter".to_string(),
        year: 1991
    };
    mybook.change(|book| {
        book.name = "Harry Potter Vol 1.".to_string();
        if book.year % 2 == 0 {
            println!("Even year!");
        } else {
            println!("Odd year!");
        }
        book.year = 1993;
    });

    println!("{:?}",mybook);
}
