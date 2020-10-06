fn main() {
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("user1: {:#?}", user1);

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
    println!(
        "build_user: {:#?}",
        build_user(String::from("test@example.com"), String::from("test"))
    );

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        // active: user1.active,
        // sign_in_count: user1.sign_in_count,
        ..user1
    };
    println!("user2: {:#?}", user2);

    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: {:#?}\norigin: {:#?}", black, origin);
}
