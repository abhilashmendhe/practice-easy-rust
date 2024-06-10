// 90. Iters/closures - .find() and .position()

/*
    .find()     - returns Some(val) where val is the first item it's mathces, else None  
    .position() - returns Some(index) where index is positon of item, else None.
*/
fn main() { 
    
    let num_vec = vec![10,20,30,40,50,60,70,80,90,100];
    
    println!("{:?}",num_vec.iter().find(|&number| *number % 3 == 0));
    println!("{:?}",num_vec.iter().find(|&number| *number * 2 == 30));
    println!("{:?}",num_vec.iter().position(|&number| number % 3 == 0));
    println!("{:?}",num_vec.iter().position(|&number| number *3 == 0));
}
