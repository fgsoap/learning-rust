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

impl Message<'_> {
    fn call(&self) {
        println!("{:?}", &self)
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
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

    // test Option
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
    println!("{:?}, {:?}, {:?}", some_number, some_string, absent_number);

    // test match
    let my_coin = Coin::Penny;
    println!("{}", value_in_cents(my_coin));
}
