use std::fmt::Display;
fn main() {
    let s1 = String::from("Hello");
    let result;
    {
        let s2 = String::from("World");
        result = longest(s1.as_str(), s2.as_str());
        println!("{}", result);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[allow(dead_code)]
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement!{}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
