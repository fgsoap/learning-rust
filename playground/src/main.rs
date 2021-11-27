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

    let result = two_times_impl();
    assert_eq!(result(2), 4);
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

fn two_times_impl() -> impl Fn(i32) -> i32 {
    let i = 2;
    move |j| j * i
}
