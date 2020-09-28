fn main() {
    let mut counter = 0;
    let result = loop {
        println!("The value of counter is {}", counter);
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}
