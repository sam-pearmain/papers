use std::str::FromStr;
use crate::bibtex::fields::error::ParseFieldError;
use super::roman::Roman;

#[derive(Debug, Clone)]
/// A type for integers or numeral numbers
pub enum Number {
    Standard(i32), 
    Roman(Roman), 
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
pub struct PositiveNumber(Number);

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