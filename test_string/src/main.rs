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
    let s = String::from("initial contents");
    println!("String::from dada is: {}", s);

    // test push
    let mut s = String::from("foo");
    println!("before push: {}", s);
    s.push_str("bar");
    s.push('s');
    println!("after push: {}", s);

    let mut s1 = String::from("foo");
    let s2= "bar";
    s1.push_str(&s2); // s1.push_str(s2)
    println!("s2 is: {}", s2);

}
