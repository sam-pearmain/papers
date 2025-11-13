use std::{fmt, str::FromStr};
use crate::bibtex::fields::error::ParseFieldError;

#[derive(Debug, Clone)]
pub enum Pages {
    /// Represents a single page, e.g., "123"
    Single(usize), 
    /// Represents a continuous range of pages, e.g., "123--145"
    Range { from: usize, to: usize }, 
    /// Represents a non-numeric page, or list
    Other(String),
}

impl FromStr for Pages {
    type Err = ParseFieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        // try to pass a single number
        if let Ok(page) = s.parse::<usize>() {
            return Ok(Pages::Single(page))
        }

        // try to pass a range
        if let Some((from_str, to_str)) = s.split_once("--") {
            if let (Ok(from), Ok(to)) = (from_str.parse::<usize>(), to_str.parse::<usize>()) {
                if to < from {
                    return Err(ParseFieldError::MangledPageRange { from, to })
                }
                return Ok(Pages::Range { from, to })
            }
        }

        // otherwise just give up
        Ok(Pages::Other(s.to_string()))
    }
}

impl fmt::Display for Pages {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pages::Single(n) => { write!(f, "{}", n) },
            Pages::Range { from, to } => { write!(f, "{}--{}", from, to) }
            Pages::Other(s) => { write!(f, "{}", s) },
        }
    }
}