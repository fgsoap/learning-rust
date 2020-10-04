fn main() {
    let s = String::from("Hello World");
    let ss = "Hi World";
    let s_word = first_word(&s);
    let ss_word = first_word(&ss);

    println!("{}, {}", s_word, ss_word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}
