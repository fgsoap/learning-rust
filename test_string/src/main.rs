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
    let s2 = "bar";
    // s1.push_str(&s2);
    s1.push_str(s2);
    println!("s2 is: {}", s2);

    // test `+` & `format!`
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s3 is: {}", s3);
    println!("{}", s2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("after s is: {}", s);

    // try slice if String
    let hello = "你好";
    let s = &hello[0..6];
    println!("{}", s);

    // try loop
    println!("loop char starts: ======");
    for c in hello.chars() {
        println!("{}", c);
    }
    println!("loop chat ends: ======");

    println!("loop byte starts: ======");
    for c in hello.bytes() {
        println!("{}", c);
    }
    println!("loop byte ends: ======");
}
