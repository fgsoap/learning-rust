fn main() {
    let mut s = String::from("Hello World");

    let word = first_word(&s); // get 5

    s.clear(); // clear s

    println!("{}", word); // word is useless as s is cleared
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
