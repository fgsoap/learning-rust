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

    statement_expression();
}

fn another_function(x: i32, y: i64) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn statement_expression() {
    let x = 2;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
