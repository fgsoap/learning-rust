fn main() {
    // println!("Hello, world!");
    // let v = "hello world!";
    // assert_eq!(v, "hello world!");
    // let v = "hello Rust!";
    // assert_eq!(v, "hello Rust!");
    // {
    //     let v = "hello World!";
    //     assert_eq!(v, "hello World!");
    // }
    // assert_eq!(v, "hello Rust!");

    // let a = 2;
    // let b = 3;
    // assert_eq!(math(sum, a, b), 5);
    // assert_eq!(math(product, a, b), 6);
    // assert_eq!(true_maker()(), true);

    // let arr = [0; init_len()];
    // println!("{:?}", arr);

    // let out = 42;
    // fn add(i: i32, j: i32) -> i32 {
    //     i + j
    // }
    // let closure_annotated = |i: i32, j: i32| -> i32 { i + j + out };
    // let closure_inferred = |i, j| i + j + out;
    // let i = 1;
    // let j = 2;
    // assert_eq!(3, add(i, j));
    // assert_eq!(45, closure_annotated(i, j));
    // assert_eq!(45, closure_inferred(i, j));

    // let a = 2;
    // let b = 3;
    // assert_eq!(closure_math(|| a + b), 5);
    // assert_eq!(closure_math(|| a * b), 6);

    // let result = two_times_impl();
    // assert_eq!(result(2), 4);

    // let n = 13;
    // let big_n = if n < 10 && n > -10 { 10 * n } else { n / 2 };
    // assert_eq!(big_n, 6);

    // let y = while_true(5);
    // assert_eq!(y, 6);

    // let number = 42;
    // match number {
    //     0 => println!("zero"),
    //     1 | 2 => println!("one or two"),
    //     3..=5 => println!("three through five"),
    //     n @ 42 => println!("the answer is {}", n),
    //     _ => println!("other"),
    // }

    // let boolean = true;
    // let mut binary = 0;
    // if let true = boolean {
    //     binary = 1;
    // }
    // assert_eq!(binary, 1);

    // let mut v = vec![1, 2, 3, 4, 5];
    // loop {
    //     match v.pop() {
    //         Some(x) => println!("{}", x),
    //         None => break,
    //     }
    // }

    // let mut v = vec![1, 2, 3, 4, 5];
    // while let Some(x) = v.pop() {
    //     println!("{}", x);
    // }

    // let _num = 42u32;
    // let _num = 0x2A;
    // let _num = 0o106;
    // let _num = 0b1101_1011;
    // assert_eq!(b'*', 42_u8); //字节字面量存储为u8, 字节字面量的表示方式为b'X', 其中X为单个ASCII字符
    // assert_eq!(b'\'', 39u8);
    // assert_eq!(2., 2.0f64);
    // assert_eq!(2e4, 20000f64);

    // // Unicode & ASCII values
    // let _x = 'r';
    // println!("{}", '\'');
    // assert_eq!('\x2A', '*'); //ASCII码16进制数
    // assert_eq!('\x25', '%');
    // assert_eq!('\u{2A}', '*'); //Unicode码16进制数
    // assert_eq!('%' as i8, 37); //char转换为i8

    // let _arr = [1, 2, 3];
    // let mut mut_arr = [1, 2, 3];
    // assert_eq!(1, mut_arr[0]);
    // mut_arr[0] = 3;
    // let init_arr = [0; 10];
    // assert_eq!(0, init_arr[5]);
    // assert_eq!(10, init_arr.len());

    // assert_eq!((1..5), std::ops::Range { start: 1, end: 5 });
    // assert_eq!((1..=5), std::ops::RangeInclusive::new(1, 5));
    // assert_eq!(3 + 4 + 5, (3..6).sum());
    // assert_eq!(3 + 4 + 5 + 6, (3..=6).sum());
    // for i in 1..5 {
    //     println!("{}", i);
    // }
    // for i in 1..=5 {
    //     println!("{}", i);
    // }

    // let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // assert_eq!(&arr, &[1, 2, 3, 4, 5]);
    // assert_eq!(&arr[1..], &[2, 3, 4, 5]);
    // assert_eq!(&arr.len(), &5);
    // assert_eq!(&arr.is_empty(), &false);
    // assert_eq!(&arr.first(), &Some(&1));
    // assert_eq!(&arr.last(), &Some(&5));
    // let arr = &mut [1, 2, 3];
    // arr[1] = 7;
    // assert_eq!(arr, &[1, 7, 3]);
    // let vec = vec![1, 2, 3];
    // assert_eq!(&vec[..], vec![1, 2, 3]);

    // let truth: &'static str = "Rust is an elgent language";
    // let ptr = truth.as_ptr();
    // let len = truth.len();
    // assert_eq!(26, len);
    // let s = unsafe {
    //     let slice = std::slice::from_raw_parts(ptr, len);
    //     std::str::from_utf8(slice)
    // };
    // assert_eq!(s, Ok(truth));

    // let mut x = 10;
    // let ptr_x = &mut x as *mut i32;
    // let y = Box::new(20);
    // let ptr_y = &*y as *const i32;
    // unsafe {
    //     *ptr_x += *ptr_y;
    // }
    // assert_eq!(x, 30);

    // let num: Option<u32> = Some(42);
    // match num {
    //     Some(num) => println!("{}", num),
    //     None => panic!("Nothing!"),
    // };

    // let tuple = ("hello", 4, 'c');
    // assert_eq!(tuple.0, "hello");
    // let coords = (0, 1); // it has Copy Trait
    // let result = move_coords(coords);
    // assert_eq!(result, (1, 2));
    // let (x, y) = move_coords(coords);
    // assert_eq!(x, 1);
    // assert_eq!(y, 2);

    // #[derive(Debug, PartialEq)] // Named-Field Struct
    // struct People {
    //     name: &'static str,
    //     gender: u32,
    // }

    // impl People {
    //     fn new(name: &'static str, gender: u32) -> Self {
    //         return People { name, gender };
    //     }
    //     fn name(&self) {
    //         println!("name: {:?}", self.name);
    //     }
    //     fn set_name(&mut self, name: &'static str) {
    //         self.name = name;
    //     }
    //     fn gender(&self) {
    //         let gender = if self.gender == 1 { "boy" } else { "girl" };
    //         println!("gender: {:?}", gender);
    //     }
    // }

    // let mut people = People::new("Tom", 1);
    // people.name();
    // people.set_name("Jerry");
    // people.name();
    // people.gender();

    // struct Color(i32, i32, i32); // Tuple-Like Struct
    // let color = Color(0, 1, 2);
    // assert_eq!(color.0, 0);

    // struct Integer(u32); // Tuple-Like Struct New Type Mode
    // let int = Integer(10);
    // assert_eq!(int.0, 10);
    // // type MyInteger = Integer;
    // // let _ = MyInteger(20); // tuple-struct cannot be used as type

    // type Int = i32; // Type aliase
    // let new_int: Int = 10;
    // assert_eq!(new_int, 10);

    // struct Empty; // unit-struct
    // let x = Empty;
    // println!("{:p}", &x);
    // let y = x;
    // println!("{:p}", &y);
    // let z = Empty;
    // println!("{:p}", &z);

    // enum Number {
    //     Zero,
    //     One,
    //     Two,
    // }
    // let a = Number::One;
    // // let b = Number::Two;
    // // let c = Number::Zero;
    // match a {
    //     Number::Zero => println!("0"),
    //     Number::One => println!("1"),
    //     Number::Two => println!("2"),
    // }

    // enum Color {
    //     Red = 0xff0000,
    //     // Green = 0x00ff00,
    //     Blue = 0x0000ff,
    // }
    // println!("roses are #{:06x}", Color::Red as i32);
    // println!("violets are #{:06x}", Color::Blue as i32);

    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let x: fn(u8, u8, u8, u8) -> IpAddr = IpAddr::V4;
    let y: fn(String) -> IpAddr = IpAddr::V6;
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!(
        "{:?}, {:?}, {:?}, {:?}",
        x(192, 168, 1, 1),
        y(String::from("V6")),
        home,
        loopback
    );

    enum Optionial {
        Some(i32),
        None,
    }
    let s = Optionial::Some(42);
    let _none = Optionial::None;
    // let num = s.unwrap();
    match s {
        Optionial::Some(n) => println!("num is: {}", n),
        Optionial::None => println!("None"),
    }
}

// pub fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
//     op(a, b)
// }

// fn sum(a: i32, b: i32) -> i32 {
//     a + b
// }
// fn product(a: i32, b: i32) -> i32 {
//     a * b
// }
// fn is_true() -> bool {
//     true
// }
// fn true_maker() -> fn() -> bool {
//     is_true
// }

// Compile-Time Function Execution
// Rust 2015
// #![feature(const_fn)]
// const fn init_len() -> usize {
//     5
// }

// fn closure_math<F: Fn() -> i32>(op: F) -> i32 {
//     op()
// }

// fn two_times_impl() -> impl Fn(i32) -> i32 {
//     let i = 2;
//     move |j| j * i
// }

// fn while_true(x: i32) -> i32 {
//     while true {
//         return x + 1;
//     }
//     x
// }

// #![feature(never_type)]
// fn foo() -> u32 {
//     let x: != {
//         return 123
//     };
// }

// fn move_coords(x: (i32, i32)) -> (i32, i32) {
//     (x.0 + 1, x.1 + 1)
// }
