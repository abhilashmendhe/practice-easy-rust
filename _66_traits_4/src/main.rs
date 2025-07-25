struct Monster {
    health: i32
}
struct Wizard {}
struct Ranger {}

trait FightClose {
    fn attack_with_sword(&self, opponent: &mut Monster){
        opponent.health -= 10;
        println!("You attack with sword. Your opponent has {} health left.",opponent.health);
    }

    fn attack_with_hand(&self, opponent: &mut Monster) {
        opponent.health -= 2;
        println!("You attack with hand. Your opponent has {} health left.",opponent.health);
    }
    
}

impl FightClose for Wizard {}
impl FightClose for Ranger {}

trait FightFromDistance {
    fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
        if distance < 10 {
            opponent.health -= 10;
            println!("You attack with bow. Your opponent has {} health left.",opponent.health);
        }
    }
    fn attack_with_rock(&self, opponent: &mut Monster, distance: u32) {
        if distance < 3 {
            opponent.health -= 4;
            println!("You attack with rock. Your opponent has {} health left.",opponent.health);
        }
    }
}

impl FightFromDistance for Ranger {}

fn main() {
    let radagest = Wizard{};
    let aragon = Ranger{};

    let mut uruk_hai = Monster { health: 40 };

    radagest.attack_with_sword(&mut uruk_hai);
    aragon.attack_with_bow(&mut uruk_hai, 5);
}
