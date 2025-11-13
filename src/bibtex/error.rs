use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParseErrorType {
    UnexpectedChar { c: char}, 
    UnknownEntry { entry: String },
    UnknownField { field: String }, 
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
            Self::UnexpectedChar { c } => {
                write!(f, "unexpected token: {}", c)
            }
        }
    }
}

#[derive(Debug)]
pub enum BibliographyError {
    EntryNotFound { citekey: String }, 
}

impl fmt::Display for BibliographyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EntryNotFound { citekey } => {
                write!(f, "entry not found: {}", citekey)
            }
        }
    }
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

    pub fn unexpected_char(c: char, row: usize, col: usize) -> Self {
        ParseError {
            kind: ParseErrorType::UnexpectedChar { c }, 
            row, col
        }
    }

    pub fn unknown_entry(entry: String, row: usize, col: usize) -> Self {
        ParseError { 
            kind: ParseErrorType::UnknownEntry { entry }, 
            row, col
        }
    }

    pub fn unknown_field(field: String, row: usize, col: usize) -> Self {
        ParseError {
            kind: ParseErrorType::UnknownField { field }, 
            row, col
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[row: {}, col {}] parse error \"{}\"", self.row, self.col, self.kind)
    }
}

impl Error for ParseError {}