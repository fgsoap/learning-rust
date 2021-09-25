#[cfg(test)]
mod tests {
    use crate::eat_at_restaurant;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn eat_at_restaurant_test() {
        eat_at_restaurant();
    }
}

mod front_of_house;

pub use crate::front_of_house::hosting;
// use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
