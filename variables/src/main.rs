use std::cmp::Ordering;

fn main() {
    let x = 6;
    println!("The value of x is: {}", x);
    match 5.cmp(&x) {
        Ordering::Equal => {
            println!("Equal!");
        }
        Ordering::Less => {
            println!("Less!");
        }
        _ => {}
    }
    println!("The value of x is: {}", x);
}