#![allow(unused_imports)]
pub use number::{Number, PositiveNumber};
pub use roman::Roman;

mod number;
mod roman;
mod error {
    use std::fmt;

    #[derive(Debug, Clone)]
    pub enum ParseRomanError {
        InvalidCharacter(char), 
        EmptyInput, 
        OutsideRange, 
    }

    impl fmt::Display for ParseRomanError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::InvalidCharacter(c) => {
                    write!(f, "invalid roman numeral character '{}'", c)
                }, 
                Self::EmptyInput => {
                    write!(f, "no numerals detected in input &str")
                },
                Self::OutsideRange => {
                    write!(f, "roman numerals can only represent 1-3999")
                }, 

            }
        }
    }

    impl std::error::Error for ParseRomanError {}
}
