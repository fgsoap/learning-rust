// use std::fs::File;
use std::io;
// use std::io::ErrorKind;
// use std::io::Read;
use std::error::Error;
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    // let f = File::open("hello.txt");
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => {
    //         // println!("{:?}", e);
    //         return Err(e);
    //     }
    // };

    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    fs::read_to_string("hello.txt")
}

fn main() -> Result<(), Box<dyn Error>> {
    // let v = vec![1, 2, 3];
    // v[99];
    // println!("Hello, world!");

    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
    //         },
    //         other_error => panic!("There wa s problem opening the file: {:?}", other_error),
    //     },
    // };

    let f = read_username_from_file()?;
    println!("{:?}", f);
    Ok(())
}
