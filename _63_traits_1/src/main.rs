// 63. Intro to traits.
/*
One type can have multiple types.
*/
struct Animal {
    name: String
}
trait Canine {
    fn bark(&self) {
        println!("Woof woof!");
    }
    fn run(&self) {
        println!("The dog is running.");
    }
    // fn legs(&self);
}

impl Canine for Animal {
    fn run(&self) {
        println!("{} is running!",self.name);
    }
    
}
fn main() {
    let rover = Animal {
        name: "Rover".to_string()
    };
    rover.bark();
    rover.run();
}
