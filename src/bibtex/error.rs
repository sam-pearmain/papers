use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParseErrorType {
    EmptyInput,
    UnexpectedChar { c: char}, 
    UnexpectedEof, 
    UnknownEntry { entry: String },
    UnknownField { field: String }, 
}

impl fmt::Display for ParseErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => {
                write!(f, "no entries found in input")
            }
            Self::UnexpectedChar { c } => {
                write!(f, "unexpected token: {}", c)
            }, 
            Self::UnexpectedEof => {
                write!(f, "unexpected eof")
            }, 
            Self::UnknownEntry { entry } => {
                write!(f, "unknown entry: {}", entry)
            }, 
            Self::UnknownField { field } => {
                write!(f, "unknown field: {}", field)
            }, 
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
    row: Option<usize>, 
    col: Option<usize>, 
}

impl ParseError {
    pub fn new(kind: ParseErrorType, row: usize, col: usize) -> Self {
        ParseError { kind, row: Some(row), col: Some(col) }
    }

    pub fn empty_input() -> Self {
        ParseError { 
            kind: ParseErrorType::EmptyInput, 
            row: None, col: None 
        }
    }

    pub fn unexpected_char(c: char, row: usize, col: usize) -> Self {
        ParseError {
            kind: ParseErrorType::UnexpectedChar { c }, 
            row: Some(row), col: Some(col)
        }
    }

    pub fn unexpected_eof(row: usize, col: usize) -> Self {
        ParseError { 
            kind: ParseErrorType::UnexpectedEof, 
            row: Some(row), col: Some(col)
        }
    }

    pub fn unknown_entry(entry: String, row: usize, col: usize) -> Self {
        ParseError { 
            kind: ParseErrorType::UnknownEntry { entry }, 
            row: Some(row), col: Some(col)
        }
    }

    pub fn unknown_field(field: String, row: usize, col: usize) -> Self {
        ParseError {
            kind: ParseErrorType::UnknownField { field }, 
            row: Some(row), col: Some(col)
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.row.is_some() && self.col.is_some() {
            write!(
                f, "[row: {}, col: {}] parse error <{}>", 
                self.row.unwrap(), self.col.unwrap(), self.kind
            )
        } else {
            write!(f, "parse error <{}>", self.kind)
        }
    }
}

impl Error for ParseError {}