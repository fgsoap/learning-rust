#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message<'t> {
    Quit,
    Move { x: i32, y: i32 },
    Write(&'t str),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));
    println!("{:?}, {:?}", four, six);
    println!(
        "{:?}, {:?}, {:?}, {:?}",
        Message::Quit,
        Message::Move { x: 23, y: 34 },
        Message::Write("Hello"),
        Message::ChangeColor(12, 23, 45)
    );

    let m = Message::Write("Just for test!");
    m.call();
    let c = Message::ChangeColor(23, 34, 56);
    c.call();
}

impl Message<'_> {
    fn call(&self) {
        println!("{:?}", &self)
    }
}
