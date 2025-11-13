use std::{fmt, str::FromStr};
use crate::bibtex::fields::error::ParseFieldError;

#[derive(Debug, Clone, Copy)]
pub enum Month {
    Jan, Feb, Mar, Apr, May, Jun, 
    Jul, Aug, Sep, Oct, Nov, Dec,
}

impl FromStr for Month {
    type Err = ParseFieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "1" | "jan" | "january" => Ok(Self::Jan),
            "2" | "feb" | "february" => Ok(Self::Feb),
            "3" | "mar" | "march" => Ok(Self::Mar),
            "4" | "apr" | "april" => Ok(Self::Apr),
            "5" | "may" => Ok(Self::May),
            "6" | "jun" | "june" => Ok(Self::Jun),
            "7" | "jul" | "july" => Ok(Self::Jul),
            "8" | "aug" | "august" => Ok(Self::Aug),
            "9" | "sep" | "september" => Ok(Self::Sep),
            "10" | "oct" | "october" => Ok(Self::Oct),
            "11" | "nov" | "november" => Ok(Self::Nov),
            "12" | "dec" | "december" => Ok(Self::Dec),
            _ => Err(ParseFieldError::UnknownMonth { got: s.to_string() }),
        }
    }
}