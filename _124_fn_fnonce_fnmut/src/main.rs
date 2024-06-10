// 124. Closures - Fn, FnMut, FnOnce
/*
    Fn (gnerally used this) - Can take by reference. Very powerfull.
    FnOnce                  - Uses what is passed in and drops it(destroys it).
    FnMut                   - Can modify it.
*/

fn main() {

    let mut my_str = String::from("Let's test these closures");

    let refs_it = || println!("{}",my_str);
    refs_it();

    let mut mutes_it = || {
        my_str.push('!');
        println!("{}",my_str);
    };
    mutes_it();
    
    let drops_it = || drop(my_str);
    drops_it();
    // println!("{}",my_str);
}
