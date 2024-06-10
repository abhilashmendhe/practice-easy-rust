// 103. anon lifetimes
/*
    error you might get "implicit elided lifetme not allowed here"
    implicit = already known
    elided   = not written
*/
struct Adventurer<'a> {
    name: &'a str,
    hit_points: u32
}

impl Adventurer<'_> {
    fn take_damage(&mut self) {
        self.hit_points -= 20;
        println!("{} has {} hit points left!",self.name, self.hit_points);
        self.name = "haha"
    }
}

fn main() {
    let mut adv1 = Adventurer {
        name: "abhi",
        hit_points: 100
    };
    adv1.take_damage();
    adv1.take_damage();
    adv1.take_damage();
}
