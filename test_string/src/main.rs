fn main() {
    // let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    println!("data to string: {}", s);
    let s = "initial contents".to_string();
    println!("literal to string: {}", s);
    // test char
    let s = 'h';
    println!("charactor is: {}", s);
}
