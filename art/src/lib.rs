//! # Art
//!
//! a libarary for art model

pub mod kinds {
    /// RYB original
    #[derive(PartialEq)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// RYB combination
    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// mix two colours to combination

    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        if c1 == PrimaryColor::Blue && c2 == PrimaryColor::Red {
            return SecondaryColor::Purple;
        } else if c1 == PrimaryColor::Blue && c2 == PrimaryColor::Yellow {
            return SecondaryColor::Green;
        }
        SecondaryColor::Orange
    }
}
