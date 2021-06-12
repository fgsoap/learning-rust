use add_one;
// use adder;
// use rand;
mod common;

// #[test]
// fn it_adds_two() {
//     common::setup();
//     assert_eq!(adder::add_two(2), 4);
// }

#[test]
fn test_add_one() {
    assert_eq!(add_one::add_one(1), 2)
}
