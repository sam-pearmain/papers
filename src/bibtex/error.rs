use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParseErrorType {
    UnknownEntry { entry: String },
    UnknownField { field: String }, 
    BraceLevelExceeded,
}

impl fmt::Display for ParseErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownEntry { entry } => {
                write!(f, "unknown entry: {}", entry)
            }, 
            Self::UnknownField { field } => {
                write!(f, "unknown field: {}", field)
            }, 
            Self::BraceLevelExceeded => {
                write!(f, "brace level exceeded")
            }
        }
    }
}

macro_rules! parse_error_constuctor_impls {
    () => {
        impl ParseError {
            pub fn 
        }
    };
}

#[derive(Debug)]
pub struct ParseError {
    kind: ParseErrorType, 
    row: usize, 
    col: usize, 
}

impl ParseError {
    pub fn new(kind: ParseErrorType, row: usize, col: usize) -> Self {
        ParseError { kind, row, col }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[row: {}, col {}] parse error \"{}\"", self.row, self.col, self.kind)
    }
}

impl Error for ParseError {}