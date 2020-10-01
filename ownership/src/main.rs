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
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}