// 67. Traits bounds
/*
Use traits to place limitation.....
*/

use std::fmt::Debug;

struct Monster{
    health: i32
}
#[derive(Debug)]
struct Wizard {
    health: i32,
}

#[derive(Debug)]
struct Ranger {
    health: i32
}
trait Magic {}
trait FightClose {}
trait FightFromDistance {}

impl FightClose for Ranger {}
impl FightClose for Wizard {}
impl FightFromDistance for Ranger {}
impl Magic for Wizard {}

fn attack_with_bow<T: FightFromDistance + Debug>(character: &T, opponent: &mut Monster) {
    opponent.health -= 10;
    println!("You attack with bow and arrow. Your opponent has {} health left. You are now at: {:?}",opponent.health, character);
}

fn attack_with_sword<T: FightClose + Debug>(character: &T, opponent: &mut Monster) {
    opponent.health -= 10;
    println!("You attack with sword. Your opponent has {} health left. You are now at: {:?}",opponent.health, character);
}

fn fireball<T: Magic + Debug> (character: &T, opponent: &mut Monster){
    opponent.health -= 20;
    println!("\nYou raise hand and cast a fireball. 
Your opponent has {} health left. 
You are now at: {:?}",opponent.health, character)
} 
fn main() {
    let radagast = Wizard {
        health: 60
    };
    let aragon = Ranger {
        health: 80
    };
    let mut uruk = Monster {
        health: 50
    };
    attack_with_sword(&aragon, &mut uruk);
    fireball(&radagast, &mut uruk);

}
