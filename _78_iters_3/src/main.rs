#[derive(Debug,Clone)]
struct Library {
    library_type: LibraryType,
    books: Vec<String>
}
#[derive(Debug,Clone)]
enum  LibraryType {
    City,
    Country,
}

impl Library {
    fn add_book(&mut self, book: &str) {
        self.books.push(book.to_string());
    }
    fn new() -> Self {
        Self{
            library_type: LibraryType::City,
            books: vec![]
        }
    }
}

impl Iterator for Library {
    type Item = String;
                        // same as Option<String>
    fn next(&mut self) -> Option<Self::Item> {
        match self.books.pop() {
            Some(book) => Some(book + " is found!"),
            None => None
        }
    }
}
fn main() {
    let mut mylibrary = Library::new();
    mylibrary.add_book("Harry Potter vol. 1");
    mylibrary.add_book("Avengers");
    mylibrary.add_book("Pokemon");

    for book in mylibrary {
        println!("{}",book);
    }
}
