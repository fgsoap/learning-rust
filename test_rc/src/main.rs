enum List {
    Con(i32, Rc<List>),
    Nil,
}
use crate::List::{Con, Nil};
use std::rc::Rc;
fn main() {
    // println!("Hello, world!");
    // let a = Rc::new(Con(5, Rc::new(Con(10, Rc::new(Nil)))));
    // let _b = Con(3, Rc::clone(&a));
    // let _c = Con(4, Rc::clone(&a));

    let a = Rc::new(Con(5, Rc::new(Con(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Con(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Con(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
