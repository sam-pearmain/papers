use std::str::FromStr;
use crate::bibtex::fields::error::ParseFieldError;

#[derive(Debug, Clone)]
/// A type for integers or numeral read numbers
pub enum Number {
    Standard(i32), 
    Numeral(Roman), 
}

impl FromStr for Number {
    type Err = ParseFieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}