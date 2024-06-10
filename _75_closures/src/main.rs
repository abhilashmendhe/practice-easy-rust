// 75. Closures, quick intro
fn main() {
    let my_closure = || println!("This is closure");
    my_closure();

    let x = 10;
    let y = 2;
    let my_second_closure = || println!("x and y: {},{}",x,y);
    my_second_closure();

    let my_third_closure = |input: i32| println!("Input: {}",input);
    my_third_closure(23);

    let my_fourth_closure = |input: i32| {
        let number = 8;
        let other_number = 10;
        println!("The numbers all together are: {}",number + other_number + input);
    }; 
    my_fourth_closure(999);
}
