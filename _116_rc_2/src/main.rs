use std::rc::Rc;

#[derive(Debug)]
struct City {
    name: Rc<String>,
    population: u32,
    city_history: Rc<String>
}

#[derive(Debug)]
struct CityData {
    names: Vec<Rc<String>>,
    histories: Vec<Rc<String>>
}
fn main() {
    let calgary = City {
        name: Rc::new("Calgary".to_string()),
        population: 1_200_000,
        city_history: Rc::new("Calgary began as a fort called Fort Calgary...".to_string())
    };
    let canada_cities = CityData {
        names: vec![Rc::clone(&calgary.name)],
        histories: vec![Rc::clone(&calgary.city_history)]
    };
    // println!("City population - {}",calgary.population);
    // println!("City population - {}",calgary.population);
    println!("{:#?}",calgary);
    
}
