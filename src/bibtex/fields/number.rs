use std::str::FromStr;
use crate::bibtex::fields::error::ParseFieldError;


pub struct Number {
    number: isize
}

impl FromStr for Number {
    type Err = ParseFieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number = s.parse::<isize>()
            .map_err(|_| ParseFieldError::UnknownNumber { got: s.to_string() })?;
        Ok(Number { number })
    }
}