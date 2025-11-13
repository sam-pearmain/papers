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
        match s.trim().to_ascii_lowercase().as_str() {
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

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Jan => write!(f, "January"),
            Self::Feb => write!(f, "February"),
            Self::Mar => write!(f, "March"),
            Self::Apr => write!(f, "April"),
            Self::May => write!(f, "May"),
            Self::Jun => write!(f, "June"),
            Self::Jul => write!(f, "July"),
            Self::Aug => write!(f, "August"),
            Self::Sep => write!(f, "September"),
            Self::Oct => write!(f, "October"),
            Self::Nov => write!(f, "November"),
            Self::Dec => write!(f, "December"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bibtex::fields::date::Month;

    #[test]
    fn test_date() {
        let jan = "jan";
        let month = jan.parse::<Month>().expect("wtf");
        println!("{}", month);
    }
}