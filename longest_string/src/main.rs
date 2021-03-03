fn main() {
    let s1 = String::from("Hello");
    let s2 = "World";
    let result = longest(s1.as_str(), s2);
    println!("{}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
