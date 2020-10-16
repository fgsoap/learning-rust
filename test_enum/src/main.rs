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

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
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
    let my_coin = Coin::Quarter(UsState::Alabama);
    println!("{}", value_in_cents(my_coin));

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six is {:?}, none is {:?}", six, none);

    // test match ignore
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        _ => (),
    }
}
