use std::{fmt, str::FromStr};
use crate::bibtex::fields::error::ParseFieldError;
use super::roman::Roman;

#[derive(Debug, Clone)]
/// A type for integers or numeral numbers
pub enum Number {
    Standard(i32), 
    Roman(Roman), 
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Standard(n) => write!(f, "{}", n),
            Self::Roman(r) => write!(f, "{}", r),
        }
    }
}

impl FromStr for Number {
    type Err = ParseFieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if let Ok(num) = s.parse::<i32>() {
            return Ok(Number::Standard(num));
        }

        if let Ok(num) = s.parse::<Roman>() {
            return Ok(Number::Roman(num));
        } 

        Err(ParseFieldError::UnknownNumber { got: s.to_string() })
    }
}

#[derive(Debug, Clone)]
/// A number we expect to be positive
pub struct PositiveNumber(Number);

impl fmt::Display for PositiveNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for PositiveNumber {
    type Err = ParseFieldError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = Number::from_str(s)?;

        match num {
            Number::Standard(n) if n <= 0 => {
                Err(ParseFieldError::NotPositive)
            }, 
            _ => Ok(PositiveNumber(num))
        }
    }
}