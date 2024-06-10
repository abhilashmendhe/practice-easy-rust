// 64. Traits 2

/* */
#[derive(Clone)]
struct Numbers {
    num1: u8,
    num2: u8
}

fn take_number(number: Numbers){

}
fn main(){
    let my_num = Numbers {
        num1: 8,
        num2: 10
    };

    take_number(my_num.clone());
    take_number(my_num);
}