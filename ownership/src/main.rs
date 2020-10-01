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
}
