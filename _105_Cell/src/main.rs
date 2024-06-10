use std::cell::Cell;

trait AwesomeTrait {
    fn return_number(&self) -> u32;
}
#[derive(Clone,Debug)]
struct Person {
    name: String,
    num: Cell<u32>
}
impl  AwesomeTrait for Person {
    fn return_number(&self) -> u32 {
        self.num.set(self.num.get() + 123);
        self.num.get()
    }
}
fn main() {
    let p = Person {
        name: "Abhi".to_string(),
        num: Cell::new(1)
    };
    dbg!(&p);
    let s = p.return_number();
    dbg!(p);
}
