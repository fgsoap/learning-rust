fn main() {
    // on heap
    let s = String::from("hello");
    let n = s;
    println!("{}", n);
    // clone on heap
    let s1 = String::from("world");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // on stack
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // tuple with Copy trait
    let m = (1, 2, "Hello World!");
    let k = m;
    println!("m = {:?}, n = {:?}", m, k);

    // example of ownership
    let s = String::from("hello");
    takes_ownership(s);
    // not work, s is dropped
    // println!("{}", s);

    let x = 5;
    makes_copy(x);
    // still keep x
    println!("{}", x);

    // s1 get value from gives_ownership()
    let s1 = gives_ownership();
    println!("s1 is: {}", s1);

    let s2 = String::from("hello");
    println!("s2 is: {}", s2);

    let s3 = takes_and_gives_back(s2);
    println!("s3 is: {}", s3);

    // try `borrowing` instead of transfer ownership
    let s1 = String::from("hello");
    // s1 will not be dropped
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
