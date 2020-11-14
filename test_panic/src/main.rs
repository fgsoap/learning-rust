use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // let v = vec![1, 2, 3];
    // v[99];
    // println!("Hello, world!");

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There wa s problem opening the file: {:?}", other_error),
        },
    };
    println!("{:?}", f);
}
