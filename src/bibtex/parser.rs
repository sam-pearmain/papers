use std::str::{Chars, FromStr};
use std::iter::Peekable;
use crate::bibtex::bibliography::Bibliography;
use crate::bibtex::entry::{Entry, EntryKind};
use crate::bibtex::error::{ParseError, ParseErrorType};
use crate::bibtex::fields::{Field, Fields};

pub struct Parser<'a> {
    input: Peekable<Chars<'a>>, 
    row: usize, 
    col: usize, 
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser { 
            input: input.chars().peekable(), 
            row: 1, col: 1,
        }
    }

    pub fn parse(&mut self) -> Result<Bibliography, ParseError> {
        let mut bibliography = Bibliography::new();

        loop {
            if let Some(entry) = self.parse_entry()? {
                bibliography.add(entry);
            } else {
                break;
            }
        }

        if bibliography.is_empty() {
            Err(ParseError::empty_input())
        } else {
            Ok(bibliography)
        }
    }

    fn parse_entry(&mut self) -> Result<Option<Entry>, ParseError> {
        self.skip_whitespace_and_comments();
        if self.peek().is_none() {
            return Ok(None);
        }

        self.consume_char('@')?;
        let entry_kind_string = self.consume_ident()?;
        let entry_kind = EntryKind::from_str(&entry_kind_string)
            .map_err(|e| ParseError::new(
                e, self.row, self.col
            ))?;

        self.consume_char('{')?;
        let citekey = self.consume_ident()?;
        self.consume_char(',')?;
        let fields = self.parse_fields()?;
        self.consume_char('}')?;

        Ok(Some(Entry::new(citekey, entry_kind, fields)))
    }

    fn parse_fields(&mut self) -> Result<Fields, ParseError>{
        self.skip_whitespace_and_comments();

        let mut fields = Fields::new();

        loop {
            let ident = self.consume_ident()?;
            let field = Field::from_str(&ident)
                .map_err(|_| ParseError::unknown_field(ident, self.row, self.col))?;

            self.consume_char('=')?;

            let value = self.consume_value()?;
            fields.insert_field(field, value.as_str())
                .map_err(|e| ParseError::new(
                    ParseErrorType::from(e), 
                    self.col, self.row
                ))?;

            self.skip_whitespace_and_comments();
            
            match self.peek() {
                Some(&'}') => {
                    // end of an entry
                    break; 
                }, 
                Some(&',') => {
                    // there is maybe another field
                    self.advance(); 
                    
                    // handle trailing comma
                    self.skip_whitespace_and_comments();
                    if let Some(&'}') = self.peek() {
                        break;
                    }
                },
                Some(&c) => {
                    return Err(ParseError::unexpected_char(c, self.row, self.col));
                },
                None => {
                    return Err(ParseError::unexpected_eof(self.row, self.col));
                }, 
            }
        }

        Ok(fields)
    }

    /// Consumes an identifier
    fn consume_ident(&mut self) -> Result<String, ParseError> {
        self.skip_whitespace_and_comments();
        let mut ident = String::new();

        while let Some(&c) = self.peek() {
            if c.is_ascii_alphanumeric() || "_+-:.".contains(c) {
                self.advance();
                ident.push(c);
            } else {
                break;
            }
        }

        if ident.is_empty() {
            Err(ParseError::unexpected_eof(self.row, self.col))
        } else {
            Ok(ident)
        }
    }

    /// Consumes a value contained within {}s or ""s
    fn consume_value(&mut self) -> Result<String, ParseError> {
        self.skip_whitespace_and_comments();

        // next char must be either a '{' or a '"'
        match self.peek() {
            Some(&c) => {
                match c {
                    '{' => self.consume_braced_value(),
                    '"' => self.consume_quoted_value(),
                    _   => Err(ParseError::unexpected_char(c, self.row, self.col))
                }
            }, 
            None => Err(ParseError::unexpected_eof(self.row, self.col))
        }
    }

    /// Consumes a "quoted" value and returns the raw string inside
    fn consume_quoted_value(&mut self) -> Result<String, ParseError> {
        self.consume_char('"')?;

        let mut value = String::new(); 

        loop {
            match self.advance() {
                Some('"') => {
                    break;
                }, 
                Some(c) => {
                    value.push(c);
                }, 
                None => {
                    return Err(ParseError::unexpected_eof(self.row, self.col))
                }
            }
        }

        if value.is_empty() {
            Err(ParseError::empty_value(self.row, self.col))
        } else {
            Ok(value)
        }
    }

    /// Consumes a {braced} value and returns the raw string inside
    fn consume_braced_value(&mut self) -> Result<String, ParseError> {
        self.consume_char('{')?;
        
        let mut value = String::new();
        let mut brace_level = 1;

        loop {
            match self.advance() {
                Some('\\') => {
                    // this is an escape character so push it and the next char
                    value.push('\\');
                    if let Some(c) = self.advance() {
                        value.push(c);
                    } else {
                        return Err(ParseError::unexpected_eof(self.row, self.col))
                    }
                }, 
                Some('{') => {
                    brace_level += 1;
                    value.push('{');
                }, 
                Some('}') => {
                    brace_level -= 1;
                    if brace_level == 0 {
                        // we have escaped the value
                        break;
                    } else {
                        value.push('}');
                    }
                }, 
                Some(c) => {
                    value.push(c);
                }, 
                None => {
                    return Err(ParseError::unexpected_eof(self.row, self.col));
                }
            }
        }

        if value.is_empty() {
            Err(ParseError::empty_value(self.row, self.col))
        } else {
            Ok(value)
        }
    }

    /// Consumes a given char
    fn consume_char(&mut self, ch: char) -> Result<(), ParseError> {
        self.skip_whitespace_and_comments();
        
        match self.peek() {
            Some(&c) => {
                if c == ch {
                    self.advance();
                    Ok(())
                } else {
                    Err(ParseError::unexpected_char(c, self.row, self.col))
                }
            }, 
            None => Err(ParseError::unexpected_eof(self.row, self.col))
        }
    }

    /// Advances the cursor forwards and consumes the current character
    fn advance(&mut self) -> Option<char> {
        match self.input.next() {
            Some('\n') => {
                self.row += 1;
                self.col = 1;
                Some('\n')
            }
            Some(c) => {
                self.col += 1;
                Some(c)
            }
            None => None, 
        }
    }

    /// Peeks the next char
    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    /// Skips whitespace and inline comments
    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.peek() {
                Some('%') => {
                    self.advance();
                    while let Some(c) = self.advance() {
                        if c == '\n' {
                            break;
                        }
                    }
                }
                Some(c) if c.is_ascii_whitespace() => {
                    self.advance();
                }
                _ => break,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_parse_succeeds() {
        let bibtex_input = r#"
            @article{Smith2023,
                author = "John Smith",
                title = {A Paper},
                journal = {Some Journal},
                year = {2023},
                pages = {100--110}
            }

            @book{Jones2022,
                title = "A Book",
                author = "Jane Jones",
                publisher = "A Publisher",
                year = {2022}
            }
        "#;

        let mut parser = Parser::new(bibtex_input);
        let result = parser.parse();

        assert!(result.is_ok(), "Parser failed but should have succeeded");

        let bibliography = result.unwrap();
        assert!(!bibliography.is_empty(), "Bibliography should contain entries");
    }

    #[test]
    fn test_mismatched_braces_fails() {
        let bibtex_input = r#"
            @article{Smith2023,
                author = "John Smith",
                title = {A Paper
            }
        "#;

        let mut parser = Parser::new(bibtex_input);
        let result = parser.parse();

        assert!(result.is_err(), "Parser succeeded but should have failed");
    }

    #[test]
    fn test_missing_comma_fails() {
        let bibtex_input = r#"
            @article{Smith2023,
                author = "John Smith"
                title = {A Paper}
            }
        "#;

        let mut parser = Parser::new(bibtex_input);
        let result = parser.parse();

        assert!(result.is_err(), "Parser succeeded but should have failed");
    }
}