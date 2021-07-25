use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::ops::DerefMut;
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list = {:?}", list);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox(String::from("Rust"));
    hello(&m);
    let new_string = "My String Rust";
    hello(new_string);

    let mut n = MyBox::new(String::from("Mut Rust"));
    mut_hello(&mut n);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn mut_hello(name: &mut str) {
    println!("Hello mut, {}!", name);
}
