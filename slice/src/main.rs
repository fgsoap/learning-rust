fn main() {
    let s = String::from("Hello, world!");

    let len = s.len();

    let slice1 = &s[3..len];

    let slice2 = &s[3..];

    let slice3 = &s[0..2];

    let slice4 = &s[..];

    println!("{}, {}, {}, {}", slice1, slice2, slice3, slice4);
}
