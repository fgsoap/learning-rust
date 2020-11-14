use std::fs::File;

fn main() {
    // let v = vec![1, 2, 3];
    // v[99];
    // println!("Hello, world!");

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("There wa s problem opening the file: {:?}", error),
    };
}
