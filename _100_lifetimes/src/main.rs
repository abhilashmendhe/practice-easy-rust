// 100. lifetimes
/*

*/

// fn return_reference() -> &'static str {
//     let my_string = String::from("I am a string");
//     &my_string
// }

#[derive(Debug)]
struct City {
    name: &'static str,
    date_founded: u32
}

fn main() {
    let my_city = City {
        name: "Pune",
        date_founded: 1909
    };
    println!("{:#?}",my_city);
}
