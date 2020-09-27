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

    another_function(32, 99);

    let z = statement_expression();

    println!("The value of z is: {}", z);

    let q = plus_one(111);

    println!("The value of q is: {}", q);
}

fn another_function(x: i32, y: i64) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn statement_expression() -> i32 {
    let x = 2;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    99;
    100
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
